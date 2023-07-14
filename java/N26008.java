import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.util.StringTokenizer;

/**
 * N26008
 */
public class N26008 {
    public static final int DIVIDER= 1000000007;
    public static void main(String[] args) {
        try(
            BufferedReader br= new BufferedReader(new InputStreamReader(System.in));
            BufferedWriter bw= new BufferedWriter(new OutputStreamWriter(System.out));
            ) {
            StringTokenizer st= new StringTokenizer(br.readLine());
            
            long n= Long.parseLong(st.nextToken());
            long m= Long.parseLong(st.nextToken());
            long a= Long.parseLong(st.nextToken());
            int h= Integer.parseInt(br.readLine());
            long answer= 1;

            for(int i =0; i<n-1; i++)
                answer= (answer * m) % DIVIDER;
            
            bw.write(String.valueOf(answer));
        } catch (Exception e) {
            // TODO: handle exception
            System.out.println(e);
        }
    }
}