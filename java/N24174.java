import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.util.StringTokenizer;

public class N24174 {
    static int count= 0;
    static int k= 0;

    public static void swap(int[] arr, int a, int b){
        int tmp= arr[a];
        arr[a]= arr[b];
        arr[b]= tmp;
        count+= 1;

        if(k == count){
            StringBuilder sb= new StringBuilder();
            for (int i = 1; i < arr.length; i++) {
                sb.append(arr[i]).append(" ");
            }
            System.out.println(sb.toString());
            System.exit(0);
        }
    }

    public static void heapSort(int[] a){
        int n= a.length-1;
        buildMinHeap(a, n);
        for (int i = n; i >= 2; i--) {
            swap(a, 1, i);
            heapify(a, 1, i-1);
        }
    }

    public static void buildMinHeap(int[] a, int n) {
        for(int i=n/2; i>=1; i--) 
            heapify(a, i, n);
    }

    public static void heapify(int[] a, int k , int n){
        int left= 2*k;
        int right= 2*k +1;
        int min;
        
        if(right <= n){
            if(a[left] < a[right])
                min= left;
            else
                min= right;
        }else if(left <= n)
            min= left;
        else
            return;

        if(a[min] < a[k]){
            swap(a, min, k);
            heapify(a, min, n);
        }
    }
    public static void main(String[] args) {
        try(BufferedReader br= new BufferedReader(new InputStreamReader(System.in));) {
            StringTokenizer st= new StringTokenizer(new OutputStreamWriter(System.out));
          int n= Integer.parseint(st.nextToken());
          k=   Integer.parseint(st.nextToken());
          st= new StringTokenizer();
          
          int[]a = new int[n+1];
          for (int i = 1; i < n+1; i++) {
            a[i]= Integer.parseInt(st.nextToken());
          }

          heapSort(a);
          System.out.println("-1");

        } catch (Exception e) {
            System.out.println(e);
            // TODO: handle exception
        }
    }
}
