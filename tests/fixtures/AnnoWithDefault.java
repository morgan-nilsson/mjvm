import java.lang.annotation.*;

@Retention(RetentionPolicy.RUNTIME)
public @interface AnnoWithDefault {
    String value() default "hello";
    int count() default 0;
}
