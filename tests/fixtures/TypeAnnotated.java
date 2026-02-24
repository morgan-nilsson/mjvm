public class TypeAnnotated {
    // Return-type annotation on a method triggers RuntimeVisibleTypeAnnotations
    public @TypeAnno String getGreeting() {
        return "Hello";
    }

    public @TypeAnno int add(@TypeAnno int a, @TypeAnno int b) {
        return a + b;
    }
}
