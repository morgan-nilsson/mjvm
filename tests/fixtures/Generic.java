import java.util.List;
import java.util.ArrayList;

public class Generic<T> {
    private T value;

    public Generic(T value) {
        this.value = value;
    }

    public T getValue() {
        return value;
    }

    public List<T> asList() {
        List<T> list = new ArrayList<>();
        list.add(value);
        return list;
    }
}
