// Targeted, read-only function export for native volunteer UI hook ABI review.
//@category V2RE
import java.io.File;
import java.io.PrintWriter;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.InstructionIterator;

public class ExportVolunteerGuiHooks extends GhidraScript {
    @Override
    protected void run() throws Exception {
        String[] args = getScriptArgs();
        File folder = new File(args[0]);
        folder.mkdirs();
        DecompInterface decompiler = new DecompInterface();
        decompiler.openProgram(currentProgram);
        try {
            for (int i = 1; i < args.length; ++i) {
                Function f = getFunctionContaining(toAddr(args[i]));
                if (f == null) {
                    printerr("Not established: no function at " + args[i]);
                    continue;
                }
                String name = f.getEntryPoint().toString().toUpperCase();
                DecompileResults result = decompiler.decompileFunction(f, 60, monitor);
                try (PrintWriter out = new PrintWriter(new File(folder, name + ".c"), "UTF-8")) {
                    out.println("// Raw Ghidra pseudo-C; not a calling convention specification.");
                    out.println(result.decompileCompleted() ? result.getDecompiledFunction().getC() : result.getErrorMessage());
                }
                try (PrintWriter out = new PrintWriter(new File(folder, name + ".asm.txt"), "UTF-8")) {
                    out.println(f.getName() + " VA " + f.getEntryPoint());
                    InstructionIterator instructions = currentProgram.getListing().getInstructions(f.getBody(), true);
                    while (instructions.hasNext()) {
                        Instruction instruction = instructions.next();
                        out.println(instruction.getAddress() + " " + instruction);
                    }
                }
            }
        } finally {
            decompiler.dispose();
        }
    }
}
