public class FFITest {
    static {
        System.loadLibrary("mynativelib");
    }

    public static native String greet();

    public static void main(String... args) {
        System.out.println(greet());
    }
}
