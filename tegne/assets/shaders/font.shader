vert.spv                                                                                                                    00000010710 00000000000 0005340                                                                                                      ustar                                                                                                                                                                                                                                                          #     p                 GLSL.std.450                      main       .   1   5   9   L   Y   ^   `   f   i   k        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   modelspace_position      in_modelspace_position       worldspace_position      Constants            model_matrix            albedo_index            sampler_index        object       screenspace_position          Light             coords           color     %   WorldObject   %       world_matrix      %      lights    %      camera_position   %      time      %      light_matrices    %      cascade_splits    '   world     .   out_modelspace_position   1   out_worldspace_position  	 5   out_screenspace_position      9   out_lightspace_position   L   out_normal    Y   in_normal     ^   out_uv    `   in_uv     d   gl_PerVertex      d       gl_Position   d      gl_PointSize      d      gl_ClipDistance   d      gl_CullDistance   f         i   out_color     k   in_color      m   MaterialObject    m       albedo_tint   m      font_width    m      font_border_tint      m      font_edge     m      font_border_offset    m      font_border_width     m      font_border_edge      m      arg_1     m      arg_2     m   	   arg_3     m   
   arg_4     o   material    G            H            H         #       H               H        #   @   H        #   D   G        H          #       H         #      G  #          G  $      @   H  %          H  %       #       H  %             H  %      #   @   H  %      #   �   H  %      #   �   H  %         H  %      #   �   H  %            H  %      #   �  G  %      G  '   "       G  '   !       G  .         G  1         G  5         G  9         G  L          G  Y         G  ^         G  `         H  d              H  d            H  d            H  d            G  d      G  i         G  k         H  m       #       H  m      #      H  m      #      H  m      #      H  m      #       H  m      #   (   H  m      #   ,   H  m      #   0   H  m      #   @   H  m   	   #   P   H  m   
   #   `   G  m      G  o   "      G  o   !            !                                          
                  
   ;           +          �?                                           	      ;        	   +                  	                    !           +  !   "        #       "     $      "     %      #   
      $         &      %   ;  &   '         (            -      
   ;  -   .      ;  -   1         4         ;  4   5        7      "      8      7   ;  8   9      +     :      +     @      +     F      ;  -   L        Q   
      ;     Y        \            ]      \   ;  ]   ^         _      \   ;  _   `      +  !   b        c      b     d         c   c      e      d   ;  e   f      ;  4   i         j         ;  j   k        m   
      
      \                        n      m   ;  n   o      6               �     ;     	      ;           ;           =  
         Q               Q              Q              P                    >  	      A              =           =        	   �              >        A  (   )   '      =     *   )   =     +      �     ,   *   +   >     ,   =     /   	   O  
   0   /   /             >  .   0   =     2      O  
   3   2   2             >  1   3   =     6      >  5   6   A  (   ;   '   :      =     <   ;   =     =      �     >   <   =   A  4   ?   9      >  ?   >   A  (   A   '   :   @   =     B   A   =     C      �     D   B   C   A  4   E   9   @   >  E   D   A  (   G   '   :   F   =     H   G   =     I      �     J   H   I   A  4   K   9   F   >  K   J   A     M         =     N   M        O      "   N   T     P   O   Q     R   P       O  
   S   R   R             Q     T   P      O  
   U   T   T             Q     V   P      O  
   W   V   V             P  Q   X   S   U   W   =  
   Z   Y   �  
   [   X   Z   >  L   [   =  \   a   `   >  ^   a   =     g      A  4   h   f      >  h   g   =     l   k   >  i   l   �  8                                                          frag.spv                                                                                                                    00000012654 00000000000 0005310                                                                                                      ustar                                                                                                                                                                                                                                                          #     �                 GLSL.std.450                     main    Q   �   �   �   �   �   �   �                �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         fragment(        tint         MaterialObject           albedo_tint         font_width          font_border_tint            font_edge           font_border_offset          font_border_width           font_border_edge            arg_1           arg_2        	   arg_3        
   arg_4        material         width        edge          border_width      $   border_edge   (   border_tint   -   border_offset     2   dist      9   textures      ;   Constants     ;       model_matrix      ;      albedo_index      ;      sampler_index     =   object    H   samplers      Q   in_uv     W   alpha     _   border_dist   o   border_alpha      w   overall_alpha     ~   overall_tint      �   out_color     �   Light     �       coords    �      color     �   WorldObject   �       world_matrix      �      lights    �      camera_position   �      time      �      light_matrices    �      cascade_splits    �   world     �   shadow_maps   �   framebuffer   �   in_normal     �   in_color      �   in_modelspace_position    �   in_worldspace_position    �   in_screenspace_position   �   in_lightspace_position  H         #       H        #      H        #      H        #      H        #       H        #   (   H        #   ,   H        #   0   H        #   @   H     	   #   P   H     
   #   `   G        G     "      G     !       G  9   "      G  9   !       H  ;          H  ;       #       H  ;             H  ;      #   @   H  ;      #   D   G  ;      G  H   "      G  H   !      G  Q         G  �          H  �       #       H  �      #      G  �          G  �      @   H  �          H  �       #       H  �             H  �      #   @   H  �      #   �   H  �      #   �   H  �         H  �      #   �   H  �            H  �      #   �  G  �      G  �   "       G  �   !       G  �   "      G  �   !       G  �   "      G  �   !       G  �          G  �         G  �         G  �         G  �         G  �              !                   	            
      	                              	      	                                       ;                       +                     	               +                       +           +     !      +     %      +     )         ,         +     .         /         +     3     �? 	 4                              5           +  5   6   d     7   4   6      8       7   ;  8   9         :           ;   :            <   	   ;   ;  <   =   	      >   	         A       4     D   +  5   E        F   D   E      G       F   ;  G   H          K       D     N   4      P         ;  P   Q      +  5   T      +     �   
�#<  �      �         ;  �   �        �         +  5   �        �   �   �     �   :   �     �   :   �   	      �         �      �   ;  �   �        �   4   T      �       �   ;  �   �       ;  A   �          �      	   ;  �   �         �         ;  �   �      ;  �   �      ;  �   �      ;  �   �        �      �      �      �   ;  �   �      6               �     9     �      �  8  6               �     ;  
         ;           ;           ;            ;     $      ;  
   (      ;  ,   -      ;     2      ;     W      ;     _      ;     o      ;     w      ;  
   ~      A              =  	         >        A              =           >        A              =           >        A     "      !   =     #   "   >      #   A     &      %   =     '   &   >  $   '   A     *      )   =  	   +   *   >  (   +   A  /   0      .   =     1   0   >  -   1   A  >   ?   =      =     @   ?   A  A   B   9   @   =  4   C   B   A  >   I   =   )   =     J   I   A  K   L   H   J   =  D   M   L   V  N   O   C   M   =     R   Q   W     S   O   R   Q     U   S      �     V   3   U   >  2   V   =     X      =     Y      =     Z      �     [   Y   Z   =     \   2        ]      1   X   [   \   �     ^   3   ]   >  W   ^   A  >   `   =      =     a   `   A  A   b   9   a   =  4   c   b   A  >   d   =   )   =     e   d   A  K   f   H   e   =  D   g   f   V  N   h   c   g   =     i   Q   =     j   -   �     k   i   j   W     l   h   k   Q     m   l      �     n   3   m   >  _   n   =     p       =     q       =     r   $   �     s   q   r   =     t   _        u      1   p   s   t   �     v   3   u   >  o   v   =     x   W   =     y   W   �     z   3   y   =     {   o   �     |   z   {   �     }   x   |   >  w   }   =  	      (   =  	   �      =     �   W   =     �   w   �     �   �   �   P  	   �   �   �   �     	   �      .      �   �   >  ~   �   =     �   w   �  �   �   �   �   �  �       �  �   �   �   �  �   �  �  �   =  	   �   ~   =     �   w   Q     �   �       Q     �   �      Q     �   �      P     �   �   �   �   �   >  �   �   �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      