0      #     p                 GLSL.std.450                      main       .   1   5   9   L   Y   ^   `   f   i   k        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   modelspace_position      in_modelspace_position       worldspace_position      Constants            model_matrix            albedo_index            sampler_index        object       screenspace_position          Light             coords           color     %   WorldObject   %       world_matrix      %      lights    %      camera_position   %      time      %      light_matrices    %      cascade_splits    %      pcf   '   world     .   out_modelspace_position   1   out_worldspace_position  	 5   out_screenspace_position      9   out_lightspace_position   L   out_normal    Y   in_normal     ^   out_uv    `   in_uv     d   gl_PerVertex      d       gl_Position   d      gl_PointSize      d      gl_ClipDistance   d      gl_CullDistance   f         i   out_color     k   in_color      m   MaterialObject    m       arg_1     m      arg_2     m      arg_3     m      arg_4     m      arg_5     m      arg_6     m      arg_7     m      arg_8     o   material    G            H            H         #       H               H        #   @   H        #   D   G        H          #       H         #      G  #          G  $      @   H  %          H  %       #       H  %             H  %      #   @   H  %      #   �   H  %      #   �   H  %         H  %      #   �   H  %            H  %      #   �  H  %      #   �  G  %      G  '   "       G  '   !       G  .         G  1         G  5         G  9         G  L          G  Y         G  ^         G  `         H  d              H  d            H  d            H  d            G  d      G  i         G  k         H  m       #       H  m      #      H  m      #       H  m      #   0   H  m      #   @   H  m      #   P   H  m      #   `   H  m      #   p   G  m      G  o   "      G  o   !            !                                          
                  
   ;           +          �?                                           	      ;        	   +                  	                    !           +  !   "        #       "     $      "    	 %      #   
      $            &      %   ;  &   '         (            -      
   ;  -   .      ;  -   1         4         ;  4   5        7      "      8      7   ;  8   9      +     :      +     @      +     F      ;  -   L        Q   
      ;     Y        \            ]      \   ;  ]   ^         _      \   ;  _   `      +  !   b        c      b     d         c   c      e      d   ;  e   f      ;  4   i         j         ;  j   k       
 m                              n      m   ;  n   o      6               �     ;     	      ;           ;           =  
         Q               Q              Q              P                    >  	      A              =           =        	   �              >        A  (   )   '      =     *   )   =     +      �     ,   *   +   >     ,   =     /   	   O  
   0   /   /             >  .   0   =     2      O  
   3   2   2             >  1   3   =     6      >  5   6   A  (   ;   '   :      =     <   ;   =     =      �     >   <   =   A  4   ?   9      >  ?   >   A  (   A   '   :   @   =     B   A   =     C      �     D   B   C   A  4   E   9   @   >  E   D   A  (   G   '   :   F   =     H   G   =     I      �     J   H   I   A  4   K   9   F   >  K   J   A     M         =     N   M        O      "   N   T     P   O   Q     R   P       O  
   S   R   R             Q     T   P      O  
   U   T   T             Q     V   P      O  
   W   V   V             P  Q   X   S   U   W   =  
   Z   Y   �  
   [   X   Z   >  L   [   =  \   a   `   >  ^   a   =     g      A  4   h   f      >  h   g   =     l   k   >  i   l   �  8  x8      #     8             2        GLSL.std.450                     main    X   n   x  �  %  .  2  7               �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         tex(i1;vf2;      index        uv       Light            coords          color        phong_dir_light(struct-Light-vf4-vf41;       light        phong_point_light(struct-Light-vf4-vf41;         light    
    phong(struct-Light-vf4-vf41;         light     "   tex_sm(i1;vf3;        index     !   uvc   &   tex_pcfsm(i1;vf3;     $   index     %   uvc   *   tex_shadow(i1;vf3;    (   index     )   uvc   .   tex_coord(i1;     -   index    
 2   shadow(struct-Light-vf4-vf41;     1   light     4   fragment(     ;   textures      D   samplers      F   Constants     F       model_matrix      F      albedo_index      F      sampler_index     H   object    V   normal    X   in_normal     [   light_dir     c   cam_dir   d   Light     d       coords    d      color     h   WorldObject   h       world_matrix      h      lights    h      camera_position   h      time      h      light_matrices    h      cascade_splits    h      pcf   j   world     n   in_modelspace_position    s   diff      y   reflect_dir   ~   spec      �   diffuse   �   specular      �   normal    �   pos   �   light_dir     �   cam_dir   �   diff      �   reflect_dir   �   spec      �   distance      �   attenuation   �   diffuse   �   specular      �   param     �   param       shadow_maps     depth       texel       softness      %  x     0  y     ;  offset    g  param     i  param     n  param     p  param     u  coord     x  in_lightspace_position    �  uv    �  depth     �  depth     �  in_screenspace_position   �  blend_margin      �  cascade   �  coord     �  param     �  blend     �  param     �  param     �  next_cascade      �  next_coord    �  param     �  shadow    �  param     �  param     �  next_shadow   �  param     �  param     �  albedo_color      �  MaterialObject    �      arg_1     �     arg_2     �     arg_3     �     arg_4     �     arg_5     �     arg_6     �     arg_7     �     arg_8     �  material        light       shadow      param       phong_light     param       ambient     lighting      #  color_mix     %  in_uv     &  param     )  param     .  in_color      2  out_color     7  in_worldspace_position  G  ;   "      G  ;   !       G  D   "      G  D   !      H  F          H  F       #       H  F             H  F      #   @   H  F      #   D   G  F      G  X          H  d       #       H  d      #      G  f          G  g      @   H  h          H  h       #       H  h             H  h      #   @   H  h      #   �   H  h      #   �   H  h         H  h      #   �   H  h            H  h      #   �  H  h      #   �  G  h      G  j   "       G  j   !       G  n         G    "      G    !       G  x        G  �        H  �      #       H  �     #      H  �     #       H  �     #   0   H  �     #   @   H  �     #   P   H  �     #   `   H  �     #   p   G  �     G  �  "      G  �  !       G  %        G  .        G  2         G  7             !                                           	            
      	              !           
                                     !                       !              !  ,         !  0          	 6                              7           +  7   8   d     9   6   8      :       9   ;  :   ;          =       6     @   +  7   A        B   @   A      C       B   ;  C   D         E           F   E            G   	   F   ;  G   H   	   +     I         J   	         M       @     P   6      W         ;  W   X      +     \          ]           d         +  7   e        f   d   e     g   E   e    	 h   E   f         g            i      h   ;  i   j         k         ;  W   n         r         +     w       +     �      B+     �      +     �      ?+     �     �?+     �   �Q�=+     �   o=+  7   �        �   ,     �   w   w   w        6   e               ;          +           	 	                            
  	            +              !        +  7   K     +     Y    �A+     ]  ��@+     c     @  v     e      w     v  ;  w  x        z        +  7   }     +     �  RI�9+  7   �      ;  z  �        �        +     �     +     �  ��L= 
 �                             �     �  ;  �  �        �                d   +       ���=   $     	   ;  $  %     ;  z  .        1        ;  1  2     ;  W   7     6               �     9     6  4   �  8  6               7        7  
      �     =     <      A  =   >   ;   <   =  6   ?   >   A  J   K   H   I   =     L   K   A  M   N   D   L   =  @   O   N   V  P   Q   ?   O   =  	   R      W     S   Q   R   �  S   8  6               7        �     ;     V      ;     [      ;     c      ;  r   s      ;     y      ;  r   ~      ;     �      ;     �      =     Y   X        Z      E   Y   >  V   Z   A  ]   ^      \   =     _   ^   O     `   _   _                  a   `        b      E   a   >  [   b   A  k   l   j   I   =     m   l   =     o   n   �     p   m   o        q      E   p   >  c   q   =     t   V   =     u   [   �     v   t   u        x      (   v   w   >  s   x   =     z   [        {   z   =     |   V        }      G   {   |   >  y   }   =        c   =     �   y   �     �      �        �      (   �   w        �         �   �   >  ~   �   =     �   s   A  ]   �      �   =     �   �   O     �   �   �             �     �   �   �   >  �   �   =     �   ~   �     �   �   �   A  ]   �      �   =     �   �   O     �   �   �             �     �   �   �   >  �   �   =     �   �   =     �   �   �     �   �   �   �  �   8  6               7        �     ;     �      ;     �      ;     �      ;     �      ;  r   �      ;     �      ;  r   �      ;  r   �      ;  r   �      ;     �      ;     �      =     �   X        �      E   �   >  �   �   =     �   n   >  �   �   A  ]   �      \   =     �   �   O     �   �   �             =     �   �   �     �   �   �        �      E   �   >  �   �   A  k   �   j   I   =     �   �   =     �   �   �     �   �   �        �      E   �   >  �   �   =     �   �   =     �   �   �     �   �   �        �      (   �   w   >  �   �   =     �   �        �   �   =     �   �        �      G   �   �   >  �   �   =     �   �   =     �   �   �     �   �   �        �      (   �   w        �         �   �   >  �   �   A  ]   �      \   =     �   �   O     �   �   �             =     �   �   �     �   �   �        �      B   �   >  �   �   =     �   �   �     �   �   �   �     �   �   �   =     �   �   =     �   �   �     �   �   �   �     �   �   �   �     �   �   �   �     �   �   �   >  �   �   =     �   �   A  ]   �      �   =     �   �   O     �   �   �             �     �   �   �   >  �   �   =     �   �   �     �   �   �   A  ]   �      �   =     �   �   O     �   �   �             �     �   �   �   >  �   �   =     �   �   =     �   �   �     �   �   �   >  �   �   =     �   �   =     �   �   �     �   �   �   >  �   �   =     �   �   =     �   �   �     �   �   �   �  �   8  6               7        �     ;     �      ;     �      A  r   �      \   �   =     �   �   �  �   �   �   w   �  �       �  �   �   �   �  �   =     �      >  �   �   9     �      �   �  �   �  �   A  r   �      \   �   =     �   �   �  �   �   �   �   �  �       �  �   �   �   �  �   =     �      >  �   �   9     �      �   �  �   �  �   �  �   �  �   �  �  �   �  8  6     "          7         7     !   �  #   =           A  =         =  6       A  M     D     =  @       V  
        =       !   Q            Y             �    8  6     &          7     $   7     %   �  '   ;  r        ;  
        ;  r        ;  r   %     ;  r   0     ;  
   ;     >    w   =       $   A  =         =  6       A  M     D     =  @       V  
        d  	      g        \   o  	       P  	     �   �   �  	         >      A  !  "  j      =     #  "  �     $  �   #  >    $  =     &         '  &  >  %  '  �  (  �  (  �  *  +      �  ,  �  ,  =     -  %  =     .    �  �   /  -  .  �  /  )  *  �  )  =     1         2  1  >  0  2  �  3  �  3  �  5  6      �  7  �  7  =     8  0  =     9    �  �   :  8  9  �  :  4  5  �  4  =     <  %  =     =  0  P  	   >  <  =  =  	   ?    �  	   @  >  ?  >  ;  @  =     A  $   A  =   B    A  =  6   C  B  A  M   D  D     =  @   E  D  V  
  F  C  E  =     G  %   O  	   H  G  G         =  	   I  ;  �  	   J  H  I  A  r   L  %   K  =     M  L  Q     N  J      Q     O  J     P     P  N  O  M  Q     Q  P     Y     R  F  P  Q  =     S    �     T  S  R  >    T  �  6  �  6  =     U  0  �     V  U  �   >  0  V  �  3  �  5  �  +  �  +  =     W  %  �     X  W  �   >  %  X  �  (  �  *  =     Z    �     [  Z  Y  >    [  =     \         ^        \  ]  �  ^  8  6     *          7     (   7     )   �  +   ;     g     ;     i     ;     n     ;     p     A  !  a  j      =     b  a  �  �   d  b  c  �  f      �  d  e  m  �  e  =     h  (   >  g  h  =     j  )   >  i  j  9     k  "   g  i  �  k  �  m  =     o  (   >  n  o  =     q  )   >  p  q  9     r  &   n  p  �  r  �  f  �  8  6     .       ,   7     -   �  /   ;  ]   u     ;  
   �     ;  r   �     =     y  -   A  z  {  x  y  =     |  {  >  u  |  A  r   ~  u  }  =       ~       �    A  r   �  u  }  >  �  �  =     �  u  O  	   �  �  �         A  r   �  u  �   =     �  �  P  	   �  �  �  �  	   �  �  �  �  	   �  �  �   P  	   �  �   �   �  	   �  �  �  >  �  �  A  r   �  u  K  =     �  �  �     �  �  �  A  r   �  u  �   =     �  �  �     �  �  �  >  �  �  A  r   �  �  �  =     �  �  A  r   �  �  }  =     �  �  =     �  �  P     �  �  �  �  �  �  8  6     2       0   7     1   �  3   ;  r   �     ;  r   �     ;     �     ;     �     ;     �     ;  r   �     ;     �     ;     �     ;     �     ;     �     ;     �     ;  r   �     ;     �     ;     �     ;  r   �     ;     �     ;     �     A  �  �  �  K  =     �  �  >  �  �  A  !  �  j   �  K  =     �  �  �     �  �  �  >  �  �  =     �  �  A  !  �  j   �  �  =     �  �  �  �   �  �  �  �  �      �  �  �  �  �  �  >  �  \   �  �  �  �  =     �  �  A  !  �  j   �  }  =     �  �  �  �   �  �  �  �  �      �  �  �  �  �  �  >  �  �   �  �  �  �  =     �  �  A  !  �  j   �  K  =     �  �  �  �   �  �  �  �  �      �  �  �  �  �  �  >  �  I   �  �  �  �  >  �    �  �  �  �  �  �  �  �  �  �  �  �  =     �  �  >  �  �  9     �  .   �  >  �  �  A  r   �  �  K  =     �  �  �  �   �  �  �   �  �      �  �  �  �  �  �  �  w   �  �  =     �  �       �  �  =     �  �  =     �  �  A  !  �  j   �  �  =     �  �  �     �  �  �       �     1   �  w   �  >  �  �  =     �  �  �  �   �  �  w   �  �      �  �  �  �  �  �  =     �  �  >  �  �  =     �  �  >  �  �  9     �  *   �  �  �  �  �  �  =     �  �  �     �  �  �        �     '     �  >  �  �  =     �  �  >  �  �  9     �  .   �  >  �  �  =     �  �  >  �  �  =     �  �  >  �  �  9     �  *   �  �  >  �  �  =     �  �  >  �  �  =     �  �  >  �  �  9     �  *   �  �  >  �  �  =     �  �  =     �  �  =     �  �       �     .   �  �  �  �  �  �  �  �  �  �  �  8  6     4          �  5   ;  ]   �     ;          ;  r        ;          ;          ;          ;          ;  ]        ;  ]   #     ;     &     ;  
   )     A  �  �  �  \   =     �  �  O     �  �  �            Q     �  �      Q        �     Q       �     P       �       �   >  �    A      j   �   \   =  d       Q             A  ]       \   >      Q     	       A  ]   
    �   >  
  	  =         >      9       2     >      =         >      9            >      A  ]       �   =         O                     �           >      =         =         =         �           �           Q             Q             Q     !       P     "       !  �   >    "  A  J   '  H   �   =     (  '  >  &  (  =  	   *  %  >  )  *  9     +     &  )  =     ,  �  �     -  +  ,  =     /  .  �     0  -  /  >  #  0  =     3  #  =     4    �     5  3  4  >  2  5  �  8  