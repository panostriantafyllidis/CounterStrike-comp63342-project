interface A {
  public void f();
}

class B implements A {
  public void f() {
    assert false;
  }
}

class C implements A {
  public void f() {}
}

public class virtual4 {
  public static void test(A b, A c) {
    c.f();
  }
}
