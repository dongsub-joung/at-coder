import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;

public class N {
    public static void main(String[] args) {
        try(
                BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
                BufferedWriter bw = new BufferedWriter(new OutputStreamWriter(System.out));
                ) {
            String[] nk= br.readLine().split(" ");
            int n = Integer.parseInt(nk[0]);
            int k = Integer.parseInt(nk[1]);

            int[] answer = solveByQueue(n, k);
            int[] answer2 = solveByLinkedList(n, k);
            
        }catch (Exception e){
            System.out.println(e);
        }
    }

    private static int[] solveByLinkedList(int n, int k) {
    }

    private static int[] solveByQueue(int n, int k) {
    }
}
