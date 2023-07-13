// https://www.acmicpc.net/problem/10818
// 최소, 최대

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.Scanner;
import java.util.StringTokenizer;

public class N10818 {

    // https://st-lab.tistory.com/43
    public static void solution1(){
        int[] arr= {};
        int N= 0;

        try(Scanner sc= new Scanner(System.in)){
            N= sc.nextInt();
            arr= new int[N];

            for(int i=0; i<N; i++)
                arr[i]= sc.nextInt();
        }catch(Exception e){
            System.out.println(e);
        }
        
        Arrays.sort(arr);
        System.out.println(arr[0] + " " + arr[N - 1]);
    }

    public static void solution2() throws Exception{
        try(BufferedReader br= new BufferedReader(new InputStreamReader(System.in))
        ){
            int N= Integer.parseInt(br.readLine());
            StringTokenizer st= new StringTokenizer(br.readLine(), " ");
            
            int index= 0;
            int[] arr= new int[N];
            while(st.hasMoreTokens()){
                arr[index++]= Integer.parseInt(st.nextToken());
            }

            Arrays.sort(arr);
            System.out.println(arr[0] + " " + arr[N - 1]);
        }catch(Exception e){
            System.out.println(e);
        }
        
    }

    public static void solution3(){
        try(BufferedReader br= new BufferedReader(new InputStreamReader(System.in))){
            Integer.parseInt(br.readLine());
            StringTokenizer st= new StringTokenizer(br.readLine(), " ");
            
            int max = -1000001;
            int min = 1000001;

            while(st.hasMoreTokens()) {

                int val = Integer.parseInt(st.nextToken());
    
                if(val>max) {
    
                    max = val;
    
                }
    
                if(val<min) {
    
                    min = val;
    
                }
    
            }
    
            System.out.println(min + " " + max);
        } catch (Exception e) {
            // TODO: handle exception
        }
    }
}
