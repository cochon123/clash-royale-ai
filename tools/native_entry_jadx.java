import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import jadx.api.JadxArgs;
import jadx.api.JadxDecompiler;
import jadx.api.JavaClass;

/** Focused static JADX export. Never loads or executes the selected Android code. */
class native_entry_jadx {
  public static void main(String[] args) throws Exception {
    Path output = Path.of(args[1]);
    Files.createDirectories(output);
    JadxArgs settings = new JadxArgs();
    settings.setInputFiles(List.of(new File(args[0])));
    settings.setThreadsCount(1);
    try (JadxDecompiler jadx = new JadxDecompiler(settings)) {
      jadx.load();
      for (int i = 2; i < args.length; i++) {
        String selected = args[i];
        JavaClass cls = jadx.getClasses().stream()
          .filter(c -> c.getFullName().equals(selected)).findFirst().orElseThrow();
        Path destination = output.resolve(selected.replace('.', '/') + ".java");
        Files.createDirectories(destination.getParent());
        Files.writeString(destination, cls.getCode());
        System.out.println(selected + " -> " + destination);
      }
    }
  }
}
