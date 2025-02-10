import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class Main {
    public static void main(String[] args) {
        BufferedReader br= new BufferedReader(new InputStreamReader(System.in));
        float h, w, n, m;
        try {
            StringTokenizer st= new StringTokenizer(br.readLine());
            h=  Float.parseFloat(st.nextToken());
            w=  Float.parseFloat(st.nextToken());
            n=  Float.parseFloat(st.nextToken());
            m=  Float.parseFloat(st.nextToken());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }

        int result_s= Math.round(h / (n+1));
        int result_w= Math.round(w / (m+1));

        System.out.println(result_s * result_w);
    }
}
