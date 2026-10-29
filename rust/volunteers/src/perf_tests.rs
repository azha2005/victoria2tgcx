//! Controlled Win32/x86 measurements on owned synthetic memory only. These are
//! helper costs and read counts, never game FPS or multiplayer evidence.
use crate::native::Native;
use crate::snapshot::Memory;
use std::cell::Cell;
use std::hint::black_box;
use std::time::Instant;

struct Arena {
    blocks: Vec<Vec<u32>>,
}
impl Arena {
    fn alloc(&mut self, bytes: usize) -> u32 {
        let block = vec![0u32; bytes.div_ceil(4)];
        let p = block.as_ptr() as u32;
        self.blocks.push(block);
        p
    }
    fn set(&mut self, p: u32, offset: usize, value: u32) {
        let block = self
            .blocks
            .iter_mut()
            .find(|b| b.as_ptr() as u32 == p)
            .unwrap();
        block[offset / 4] = value;
    }
}
struct Fixture {
    _arena: Arena,
    native: Native,
    left: u32,
    right: u32,
    province: u32,
}
fn fixture() -> Fixture {
    let mut a = Arena { blocks: Vec::new() };
    let base = a.alloc(0xE58900);
    let world = a.alloc(0xB64);
    let countries = a.alloc(8);
    let table = a.alloc(1000 * 4);
    let relations = a.alloc(1000 * 4);
    a.set(base, 0xE588E8, world);
    a.set(base, 0xE587E4, countries);
    a.set(countries, 0, 1000);
    a.set(countries, 4, table);
    for i in 1..1000 {
        let country = a.alloc(0x1320);
        a.set(table, i * 4, country);
        a.set(country, 0x20, i as u32);
        a.set(country, 0x1C, u32::from_le_bytes(*b"TAG\0"));
        a.set(country, 0xBE8, relations);
    }
    let context = a.alloc(0x1900);
    let holder = a.alloc(0x130);
    let manager = a.alloc(0x184);
    let queue = a.alloc(0x20);
    a.set(world, 0xB60, 625);
    a.set(world, 0xB0C, 100);
    a.set(world, 0xB24, context);
    a.set(context, 0, base + 0xA0A34C);
    a.set(context, 0x17E8, holder);
    a.set(holder, 0x12C, manager);
    a.set(manager, 0, base + 0xA390D0);
    a.set(manager, 0x114, base + 0xA0A168);
    a.set(manager, 0x118, 0x18D);
    a.set(manager, 0x2C, 4);
    a.set(manager, 0x180, 2);
    a.set(manager, 0x38, queue);
    a.set(queue, 0, base + 0xA42840);
    a.set(queue, 0x1C, manager);
    let province = a.alloc(0x180);
    let map = a.alloc(0x364);
    a.set(province, 0x5C, map);
    a.set(map, 0x28, 1 << 16);
    let left = a.alloc(0x1B8);
    let right = a.alloc(0x1B8);
    for (army, id, owner) in [(left, 100, 3), (right, 101, 4)] {
        a.set(army, 0, base + 0x9F88CC);
        a.set(army, 0x10, 40);
        a.set(army, 0x14, id);
        a.set(army, 0xC4, owner);
        a.set(army, 0xCC, owner);
        a.set(army, 0xDC, province);
    }
    for (offset, rva) in [
        (0x2C, 0x1EC4A0),
        (0x38, 0x631490),
        (0x3C, 0x18CFD0),
        (0x40, 0x18CFD0),
    ] {
        a.set(base, 0x9F88CC + offset, base + rva);
    }
    Fixture {
        _arena: a,
        native: Native { base },
        left,
        right,
        province,
    }
}
struct Counted<'a> {
    memory: &'a Native,
    reads: Cell<u64>,
}
impl Memory for Counted<'_> {
    fn read32(&self, p: u32) -> Option<u32> {
        self.reads.set(self.reads.get() + 1);
        self.memory.read32(p)
    }
}
#[test]
#[ignore = "explicit release benchmark; does not run against the game"]
fn helper_benchmark() {
    let f = fixture();
    let (mut app, world) = crate::send_tests::setup(1);
    crate::send_tests::select(&mut app, &world, 1);
    let (serial, receipt) = crate::send_tests::deploy(&mut app, &world, None);
    let registry = app.registry.as_mut().unwrap();
    assert!(registry.receipt(serial, &receipt));
    let permit = crate::session::admit(world.mode, app.capabilities, world.epoch).unwrap();
    let memory = Counted {
        memory: &f.native,
        reads: Cell::new(0),
    };
    let one = crate::native_combat::evaluate(
        &memory,
        f.native.base,
        registry,
        permit,
        f.left,
        f.right,
        f.province,
    );
    assert_eq!(one, crate::combat::Decision::Native);
    let reads = memory.reads.get();
    memory.reads.set(0);
    let pair_loops = 1000000;
    let start = Instant::now();
    for _ in 0..pair_loops {
        black_box(crate::native_combat::evaluate(
            &memory,
            f.native.base,
            registry,
            permit,
            f.left,
            f.right,
            f.province,
        ));
    }
    let pair_ns = start.elapsed().as_nanos();
    assert_eq!(memory.reads.get(), reads * pair_loops);
    let full = f.native.world().unwrap();
    assert_eq!(full.countries.len(), 999);
    let world_loops = 2000;
    let start = Instant::now();
    for _ in 0..world_loops {
        black_box(f.native.world().unwrap());
    }
    let world_ns = start.elapsed().as_nanos();
    let compact = f.native.mission_world(registry).unwrap();
    assert_eq!(compact.countries.len(), 2);
    assert!(compact.armies.is_empty());
    let expected = full
        .countries
        .iter()
        .filter(|c| [625, 648].contains(&c.index))
        .map(|c| {
            (
                c.index,
                c.tag,
                &c.caption,
                c.war,
                c.hostile_to_player,
                c.landing_province,
            )
        })
        .collect::<Vec<_>>();
    let actual = compact
        .countries
        .iter()
        .map(|c| {
            (
                c.index,
                c.tag,
                &c.caption,
                c.war,
                c.hostile_to_player,
                c.landing_province,
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
    assert_eq!(
        (
            compact.player,
            compact.date,
            compact.mode,
            compact.player_at_war
        ),
        (full.player, full.date, full.mode, full.player_at_war)
    );
    let start = Instant::now();
    for _ in 0..world_loops {
        black_box(f.native.mission_world(registry).unwrap());
    }
    let compact_ns = start.elapsed().as_nanos();
    println!(
        "BENCH {{\"pair_loops\":{pair_loops},\"pair_reads\":{reads},\"pair_ns\":{pair_ns},\"world_loops\":{world_loops},\"world_countries\":{},\"world_ns\":{world_ns},\"compact_countries\":{},\"compact_ns\":{compact_ns}}}",
        full.countries.len(),
        compact.countries.len()
    );
}

#[test]
fn negative_filter_skips_only_unrelated_pairs_and_rechecks_relevant_mode() {
    let mut f = fixture();
    let (mut app, world) = crate::send_tests::setup(1);
    crate::send_tests::select(&mut app, &world, 1);
    let (serial, receipt) = crate::send_tests::deploy(&mut app, &world, None);
    let registry = app.registry.as_mut().unwrap();
    assert!(registry.receipt(serial, &receipt));
    let permit = crate::session::admit(world.mode, app.capabilities, world.epoch).unwrap();
    let m = Counted {
        memory: &f.native,
        reads: Cell::new(0),
    };
    assert_eq!(
        crate::native_combat::evaluate(
            &m,
            f.native.base,
            registry,
            permit,
            f.left,
            f.right,
            f.province
        ),
        crate::combat::Decision::Native
    );
    assert_eq!(m.reads.get(), 4);
    f._arena.set(f.left, 0x10, receipt.army.kind);
    f._arena.set(f.left, 0x14, receipt.army.id);
    // Identity is a negative filter, never proof of ownership/membership.
    m.reads.set(0);
    assert_eq!(
        crate::native_combat::evaluate(
            &m,
            f.native.base,
            registry,
            permit,
            f.left,
            f.right,
            f.province
        ),
        crate::combat::Decision::Native
    );
    assert!(m.reads.get() > 4);
    let s = crate::native_session::capture(&f.native, f.native.base).unwrap();
    f._arena.set(s.manager, 0x180, 1);
    f._arena.set(s.queue, 0, f.native.base + 0xA42728);
    assert_eq!(
        crate::native_session::capture(&f.native, f.native.base)
            .unwrap()
            .mode,
        crate::session::SessionMode::Multiplayer
    );
    m.reads.set(0);
    assert_eq!(
        crate::native_combat::evaluate(
            &m,
            f.native.base,
            registry,
            permit,
            f.left,
            f.right,
            f.province
        ),
        crate::combat::Decision::Native
    );
    assert!(m.reads.get() > 4);
}
