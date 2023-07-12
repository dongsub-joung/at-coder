import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.LinkedList;
import java.util.StringTokenizer;

public class N1021 {
    private static int N, M;
    private static int count= 0;
    private static LinkedList<Integer> queue= new LinkedList<>();
    
    public static void main(String[] args) throws Exception {
        try(BufferedReader br= new BufferedReader(new InputStreamReader(System.in))){
            StringTokenizer tokenizer= new StringTokenizer(br.readLine());
            
            N= Integer.parseInt(tokenizer.nextToken());
            M= Integer.parseInt(tokenizer.nextToken());

            tokenizer= new StringTokenizer(br.readLine());

            int[] temp= new int[M];
            for(int i=0; i< M; i++){
                temp[i] = Integer.parseInt(tokenizer.nextToken());
            }

            for(int i=1; i<=N; i++)
                queue.add(i);
        }
    }
}
