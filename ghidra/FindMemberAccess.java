// Bounded read-only field-access inventory; a matching displacement is not a type.
//@category V2RE
import java.io.PrintWriter;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.InstructionIterator;

public class FindMemberAccess extends GhidraScript {
    @Override
    protected void run() throws Exception {
        String[] args = getScriptArgs();
        String suffix = "+ 0x" + Long.toHexString(Long.decode(args[1])) + "]";
        int count = 0;
        try (PrintWriter out = new PrintWriter(args[0], "UTF-8")) {
            InstructionIterator it = currentProgram.getListing().getInstructions(true);
            while (it.hasNext() && !monitor.isCancelled()) {
                Instruction ins = it.next();
                if (!ins.toString().toLowerCase().contains(suffix)) continue;
                Function f = getFunctionContaining(ins.getAddress());
                out.println(ins.getAddress() + " " + ins + " in " +
                    (f == null ? "unknown" : f.getEntryPoint() + " " + f.getName()));
                if (++count >= 5000) { out.println("LIMIT REACHED"); break; }
            }
        }
        println("Member displacement matches: " + count + " (not type proof)");
    }
}
