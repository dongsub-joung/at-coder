 import java.io.BufferedReader;
 import java.io.InputStreamReader;
 import java.util.StringTokenizer;
 
 public class N2908
 {
     // https://www.acmicpc.net/problem/2908 상수
     public static void main( String[] args )
     {  
         int x=0;
         int y=0;
         BufferedReader reader= new BufferedReader(new InputStreamReader(System.in));
         
         try {
             StringTokenizer st = new StringTokenizer(reader.readLine());
             String str_x= st.nextToken();
             String str_y= st.nextToken();
 
             char[] chars= new char[str_x.length()];
             char[] chars2= new char[str_y.length()];;
 
             int i= 2;
             for(var c : str_x.toCharArray()){
                 chars[i]= c;
                 i--;
             }
 
             i= 2;
             for(var c : str_y.toCharArray()){
                 chars2[i]= c;
                 i--;
             }
 
             x= Integer.parseInt(new String(chars));
             y= Integer.parseInt(new String(chars2));
         } catch (Exception e) {
             e.printStackTrace();
         }
         
         if (x > y) {
             System.out.printf("%d", x);           
         }else{
             System.out.printf("%d", y);
         }
     }
 }
 