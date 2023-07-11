//https://risk-boy.github.io/boj/25556/
// 포스택
import java.util.ArrayList;
import java.util.Scanner;
import java.util.Stack;

public class Main {
    final static int SIZE_OF_STACK= 4;

    public static void main(String[] args) {
        String result= "";
        ArrayList<Stack<Integer>> stacks= new ArrayList<>();

        try(Scanner sc= new Scanner(System.in)) {
            int n= sc.nextInt();
            sc.nextLine();
            String[] strs= sc.nextLine().split(" ");

            for (int i=0; i< SIZE_OF_STACK; i++){
                stacks.add(new Stack<>());
                stacks.get(i).push(0);
            }

            System.out.println(getResult(n, strs, SIZE_OF_STACK, stacks));
        }


    }

    private static String getResult(int n, String[] strs, int sizeOfStack, ArrayList<Stack<Integer>> stacks) {
        int e;
        for (int i=0; i<n; i++){
            e= Integer.parseInt(strs[i]);

            for (int j=0; j<sizeOfStack; j++){
                if (stacks.get(j).peek() < e){
                    stacks.get(j).push(e);
                    e= 0;
                    break;
                }
            }
            if (e != 0) return "NO";
        }
        return "YES";
    }
}