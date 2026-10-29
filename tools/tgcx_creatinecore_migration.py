"""Pure settings migration for the isolated Volunteers loader upgrade.

Reads supplied text only. Installation, backups and upstream downloads belong
to the caller. Preserve existing settings rather than importing gameplay defaults.
"""
import re

NEW_DISABLED = {
    'FACTORY_MAINTENANCE_IGNORE_THROUGHPUT', 'FACTORY_SUBSIDY_COVERS_TARIFF',
    'FACTORY_DELETE_CLOSED', 'PRODUCTION_STATE_RGO_ICONS',
    'FACTORY_MAINTENANCE_DISPLAY_FIX', 'GOODS_NEED_TOTAL',
    'TRADE_WINDOW_REAL_POP_NEEDS', 'POP_BUILD_INFRASTRUCTURE_RULE',
    'ENABLE_IF_ELSE_OPERATORS', 'FIX_TECH_FACTORY_MAINTENANCE',
    'ENABLE_EXTENDED_UNIT_RECRUITMENT_RULES', 'FIX_PROVINCE_FLAGS',
}
CORE_FORCE = {
    'LOCAL_MOD_CONFIG': '0', 'LOAD_DLL_FROM_MOD_FOLDER': '1',
    'ENABLE_STATE_PROBE': '0', 'ENABLE_RNG_PROBE': '0', 'DEBUG_LOG': '0',
}
PERF_FORCE = {'FIX_ARMY_WINDOW_LAG': '0', 'DEBUG_LOG': '0'}
RETIRED = {'PATCH_GRAPH_POINT_CLAMP'}


def settings(text):
    result = {}
    for raw in text.splitlines():
        line = raw.split(';', 1)[0].strip()
        if not line or line.startswith('#'):
            continue
        key, sep, value = line.partition('=')
        key, value = key.strip(), value.strip()
        if not sep or not re.fullmatch('[A-Z][A-Z0-9_]*', key) or key in result:
            raise ValueError('invalid or duplicate setting: ' + key)
        result[key] = value
    return result


def render(template, values):
    original = settings(template)
    if original.keys() != values.keys():
        raise ValueError('template/value keys differ')
    for key, value in values.items():
        if any(c in value for c in '\r\n;'):
            raise ValueError('invalid value')
        pattern = r'(?m)^(' + re.escape(key) + r'[ \t]*=[ \t]*)[^;\r\n]*'
        template, count = re.subn(pattern, lambda match: match[1] + value, template)
        if count != 1:
            raise ValueError('expected unique template key: ' + key)
    return template


def migrate(old_text, core_template, perf_template):
    old = settings(old_text)
    core, perf = settings(core_template), settings(perf_template)
    unknown = old.keys() - core.keys() - perf.keys() - RETIRED
    if unknown:
        raise ValueError('unmapped old settings: ' + ', '.join(sorted(unknown)))
    for key in RETIRED & old.keys():
        if old[key] != '0':
            raise ValueError('cannot silently remove enabled setting: ' + key)
    for target in (core, perf):
        for key in target.keys() & old.keys():
            target[key] = old[key]
    for key in NEW_DISABLED & core.keys() - old.keys():
        core[key] = '0'
    core.update(CORE_FORCE)
    perf.update(PERF_FORCE)
    return render(core_template, core), render(perf_template, perf), {
        'retired_disabled_keys': sorted(RETIRED & old.keys()),
        'new_disabled_keys': sorted(NEW_DISABLED & core.keys() - old.keys()),
        'core_forced': CORE_FORCE, 'performance_forced': PERF_FORCE,
    }


def defines(old_text):
    """Extract values from loader-generated data, never copy a TGC source file."""
    values = {}
    for raw in old_text.splitlines():
        line = raw.split('#', 1)[0].strip()
        if not line:
            continue
        key, sep, value = line.partition('=')
        key, value = key.strip(), value.strip()
        if not sep or key in values:
            raise ValueError('invalid/duplicate define')
        values[key] = value
    defaults = {
        'factory_unprofitable_close_days': '60', 'minting_formula': '',
        'ai_factory_expand_min_staffing': '90', 'factory_closed_delete_days': '90',
        'ai_naval_base_max_provinces_per_state': '1',
        'factory_min_wage_per_10000': '7', 'pop_salary_additional_multiplier': '10',
        'price_delta_exempt_goods': '',
    }
    if values.keys() - defaults.keys():
        raise ValueError('unknown defines require review')
    defaults.update(values)
    return ('# Generated loader data for the isolated Volunteers test copy.\n'
            '# Required upstream filename: defines_ccore.txt.\n'
            '# Keep this additive carrier first in the explicit launch arguments.\n'
            + ''.join(f'{key} = {value}\n' for key, value in defaults.items()))
