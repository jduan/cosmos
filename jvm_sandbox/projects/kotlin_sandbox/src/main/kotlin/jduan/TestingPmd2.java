package jduan;

public class TestingPmd2 {
    String bill;

    public void method() {
        // This will trigger the DontCallBossShort PMD rule
        short bill; // LocalVariableDeclaration
    }
}
