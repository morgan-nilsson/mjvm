import java.io.IOException;

public class WithExceptions {
    public void riskyMethod() throws IOException, IllegalArgumentException {
    }

    public void multiThrows() throws IOException, RuntimeException, ClassNotFoundException {
    }
}
