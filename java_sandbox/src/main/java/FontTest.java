import java.awt.*;
import java.io.FileInputStream;
import java.util.zip.ZipInputStream;

public class FontTest {
  private static final String FONT_FILE = "/Users/jingjing_duan/tmp/PingFang-SC.ttf.zip";

  public static void main(String[] args) {
    try (final ZipInputStream zipInputStream =
                 new ZipInputStream(new FileInputStream(FONT_FILE))) {
      zipInputStream.getNextEntry();
      Font font = Font.createFont(Font.TRUETYPE_FONT, zipInputStream);
      System.out.println("Font created: " + font);
    } catch (Exception e) {
      System.err.println("load font failed");
      e.printStackTrace();
    }
  }
}
