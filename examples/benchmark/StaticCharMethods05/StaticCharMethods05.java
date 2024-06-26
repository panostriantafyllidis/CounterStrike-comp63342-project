import java.util.Scanner;

public class StaticCharMethods05 {
    public static void test(String userInput, int radix, int choice) {
        Scanner scanner = new Scanner(userInput);

        assert choice >= 0 && choice < 3;

        switch (choice) {
          case 1: // convert digit to character
            System.out.println("Enter a digit:");
            int digit = scanner.nextInt();
            System.out.printf("Convert digit to character: %s\n", Character.forDigit(digit, radix));
            char tmp = Character.forDigit(digit, radix);
            assert tmp == 't';
            break;

          case 2: // convert character to digit
            System.out.println("Enter a character:");
            char character = scanner.next().charAt(0);
            System.out.printf("Convert character to digit: %s\n", Character.digit(character, radix));
            break;
        }
    }
}
