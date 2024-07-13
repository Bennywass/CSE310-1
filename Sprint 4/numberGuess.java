import java.util.Scanner;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException; 


public class numberGuess{
    private static int generateRandom(){
      // Minimum value of range
      int min = 1; 
      // Maximum value of range
      int max = 100; 

      // Generate random int value from min to max
      int randomInt = (int)Math.floor(Math.random() * (max - min + 1) + min);

      return randomInt;
    }


    private static int getInput(Scanner user){
        // Get input from user
        System.out.println("Between 1 and 100 what is your guess?");
        int userGuss = user.nextInt();
        
        // return input
        return userGuss;
    }

    private static void createFile(int userGuess){
        try {
            File file = new File("guesses.txt");
            if (file.createNewFile()) {
              System.out.println("File created: " + file.getName());
            } else {
              writeFile(userGuess);
            }
          } catch (IOException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
          }
    }

    private static void writeFile(int userGuess){
        try{
            FileWriter myWriter = new FileWriter("guesses.txt");
            myWriter.write(String.valueOf(userGuess));
            myWriter.close();
            } catch (IOException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
            }
    }

    private static void tooHighOrLow(int userGuess, int randomInt){

        // If user guessed too high
        if(userGuess > randomInt){
            System.out.println("Oh, I am so sorry! You guess a bit too high!");
        }

        else{
            System.out.println("Oh, I am so sorry! You guess a bit too low!");
        }
    }

    private static void introduction(){
        System.out.println("Hello and welcome to the number guessing game!");
        System.out.println("How this works is we are going to generate a number between 1 and 100.");
        System.out.println("Your job will be to guess within 10 attempts!");
        System.out.println("Good luck!");
        System.out.println();
    }


    public static void main(String[] args){
        // Allow for user inputs
        Scanner user = new Scanner(System.in);
   
        // Initialize ints
        int randomInt = generateRandom();
        int userGuess = 0;
        int attempts = 10;
        int numOfAttempts = 0;

        // Introduce the game
        introduction();
    
        // Loop the game
        while(userGuess != randomInt && attempts > 0){
            userGuess = getInput(user);

            // Check if user won
            if(userGuess != randomInt){

                // determin if userGuess was too high or too low
                tooHighOrLow(userGuess, randomInt);
                createFile(userGuess);
                attempts--;
            }

            // Increase the number of attempts taken
            numOfAttempts++;
            System.out.println("You have " + attempts + " Left");
            System.out.println();
            }
        
        // Lose message if user doesn't guess correctly
        if(attempts == 0)
        {
            System.out.println("I'm sorry, but you are out of guesses.");
            System.out.println("Thank you for playing!");
        }

        // Congratulate user on a well deserved victory
        else
        {
        System.out.println("Congratulations! You guessed " + userGuess + "! You win! And it only took you " 
        + numOfAttempts + " tries!" );

        System.out.println("Thank you for playing!");
        }

        user.close();
    }

}

