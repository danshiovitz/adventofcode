import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.function.ToIntFunction;

public class prog {
  private static void run(String action, String filename) throws Exception {
    ToIntFunction<String> func;
    if (action.equals("raw")) {
      func = prog::rawLength;
    } else if (action.equals("decoded")) {
      func = prog::decodedLength;
    } else if (action.equals("decodedDiff")) {
      func = prog::decodedDiff;
    } else if (action.equals("encoded")) {
      func = prog::encodedLength;
    } else if (action.equals("encodedDiff")) {
      func = prog::encodedDiff;
    } else {
      throw new RuntimeException(String.format("Unknown action: %s", action));
    }

    Path path = new File(filename).toPath();
    System.out.println(String.format(
      "Total chars: %d",
      Files.lines(path).map(String::trim).mapToInt(func).sum())
    );
  }

  private static int rawLength(String line) {
    return line.length();
  }

  private static int decodedLength(String line) {
    if (line.charAt(0) != '"' || line.charAt(line.length() - 1) != '"') {
      throw new RuntimeException(String.format("Bad line quotes: %s", line));
    }

    StringBuilder builder = new StringBuilder();
    for (int i = 1; i < line.length() - 1; i++) {
      if (line.charAt(i) != '\\') {
        builder.append(line.charAt(i));
        continue;
      }
      if (i >= line.length() - 2) {
        throw new RuntimeException(String.format("Bad line short escape: %s", line));
      }
      if (line.charAt(i+1) == '\\' || line.charAt(i+1) == '"') {
        builder.append(line.charAt(i+1));
        i++;
        continue;
      } else if (line.charAt(i+1) == 'x') {
        if (i >= line.length() - 4) {
          throw new RuntimeException(String.format("Bad line hex escape: %s", line));
        }
        int hexval = Integer.parseInt(line.substring(i+2, i+4), 16);
        builder.append((char)hexval);
        i += 3;
        continue;
      } else {
        throw new RuntimeException(String.format("Bad line escape: %s", line));
      }
    }

    return builder.length();
  }

  private static int decodedDiff(String line) {
    int diff = rawLength(line) - decodedLength(line);
    if (diff < 2) {
      throw new RuntimeException(String.format("Unexpected diff (%d) for line: %s", diff, line));
    }
    return diff;
  }

  private static int encodedLength(String line) {
    StringBuilder builder = new StringBuilder();
    builder.append('"');

    for (int i = 0; i < line.length(); i++) {
      if (line.charAt(i) == '\\') {
        builder.append("\\\\");
      } else if (line.charAt(i) == '"') {
        builder.append("\\\"");
      } else {
        builder.append(line.charAt(i));
      }
    }
    builder.append('"');
    return builder.length();
  }

  private static int encodedDiff(String line) {
    int diff = encodedLength(line) - rawLength(line);
    if (diff < 2) {
      throw new RuntimeException(String.format("Unexpected diff (%d) for line: %s", diff, line));
    }
    return diff;
  }

  public static void main(String[] args) {
    try {
      run(args[0], args[1]);
    } catch (Exception ex) {
      System.out.println(String.format("Oops! %s", ex));
      ex.printStackTrace();
    }
  } 
}
