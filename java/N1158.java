import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;

public class N1158 {
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
            

            if(n == 1 && k == 1) {
                bw.write("<" + answer[0] + ">");
            }else {
                bw.write("<" + answer[0]);
                for (int i = 1; i < answer.length - 1; i++) {
                    bw.write(", " + answer[i]);
                }
                bw.write(", " + answer[answer.length-1] + ">");
            }
        }catch (Exception e){
            System.out.println(e);
        }
    }

    private static int[] solveByLinkedList(int n, int k) {
        List<Integer> list= new LinkedList<>();
        int[] answer= new int[n];
        k-= 1;
        for(int i =0; i<n; i++)
            list.add(i+1);
        
        for(int i=0; i<n; i++){
            int loop= k;
            while(loop-- > 0){
                list.add(list.remove(0));
            }
            answer[i]= list.remove(0);
        }

        return answer;
    }

    private static int[] solveByQueue(int n, int k) {
        Queue<Integer> queue= new LinkedList<>();
        int[] answer= new int[n];
        for(int i=0; i<n; i++)
            queue.add(i+1);
        int index=0;
        while(!queue.isEmpty()){
            for (int i = 0; i < k-1; i++) {
                int value = queue.remove();
                queue.add(value);
            }
            answer[index] = queue.remove();
            index++;
        }
        return answer;
    }
}
