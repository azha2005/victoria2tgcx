// Focused read-only province rename / text entry research. Private output is ignored.
//@category V2RE
import java.io.PrintWriter;
import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.*;
public class tgcx_ProvinceRenameInspect extends GhidraScript {
    public void run() throws Exception {
        String[] args=getScriptArgs();
        try(PrintWriter out=new PrintWriter(args[0], "UTF-8")) {
            for(int n=1;n<args.length;n++) {
                Function f=getFunctionContaining(toAddr(args[n]));
                if(f==null)continue;
                out.println(f.getName()+" "+f.getEntryPoint()+" "+f.getSignature());
                InstructionIterator it=currentProgram.getListing().getInstructions(f.getBody(),true);
                int count=0;
                while(it.hasNext()) {
                    Instruction ins=it.next();
                    if(count++<12 || ins.getMnemonicString().startsWith("RET"))out.println(ins.getAddress()+" "+ins);
                }
            }
        }
    }
}
