# What is Octo Sphere?
The purpose of this code is to dynamically tile a sphere using an octahedron whose faces are subdivided into equilateral triangles of equal area in such a way that allows adjacent tile movement that can cirumnavigate the sphere and end up at the origin if chained.
## Table of Contents
1. [Program Motivation](#Program-Motivation)
2. [The Octahedron](#The-Octahedron)
3. [Tile Movement](#Tile-Movement)
    - [Base Case](#Base-Case)
    - [Changing Face](#Changing-Face)
        - [Pole Movement](#Pole-Movement)
        - [Equatorial Corner](#Equatorial-Corner)
        - [Equatorial Crossing](#Equatorial-Crossing)
        - [Side Crossing](#Side-Crossing)
4. [Code Testing](#Code-Testing)
4. [Future Plans](#Future-Plans)
## Program Motivation
The classic approach to tiling spheres is done using square and mapped onto a cylinder. This has been the common approach for decades until recently. Modern tiling of spheres has moved from square tiles to hexagons, as it allows more degrees of equidistant movement.  The problem with tiling a sphere with equal sized hexagons is that it simply can't be done, most programs tile a cube with hexagons, leaving the corners as pentagons, and map this to a sphere. What if one wants all the tiles the same shape and still wants to retain the same axial movements? Consider a hexagon as the following six equilateral triangles:

    /1\2/3\
    \5/6\7/
Figure 1) A hexagon made from equilateral triangles.

Here we can see that moving through the triangles from 1 to 7 lies on the same axis. Similarly, 5 and 3 also sit on the same, albeit different, axis. Additionally, the numbers above and below one another are also along a third axis, each column being parallel to the others. All three of these axes are the same as those that would be in any hexagonal tiling. Furthermore, this hexagon can easily be made a triangle by adding a few more tiles, as shown below.

        /0\
      /1\2/3\
    /4\5/6\7/8\
Figure 2) A equilateral triangle subdivided into equilateral triangles.

This triangle can then form one of the eight faces of an octahedron. While this isn't a perfect mapping, it has some niceties in regards to local movement along the axes. Each corner vertex loses two triangles in the hexagon that would form around it (note that this actually forms a square at the points of distortion) but since they are the two triangles that don't correlate with axial movement, the can be safely disregarded.
## The Octahedron
The data structure of the polyhedron is simply a vector of tiles, with each one having a unique id calculated from which face, the size of the face, and index position on the face.  The construction takes a size that defines the face length in the number of tile sides, where a value of 1 would have faces being a single triangle. The algorithm just constructs all eight faces the same but pretends that the last four are flipped.  This creates pairs of faces as seen here:

    /0\ /1\ /2\ /3\ -> /0\
    \7/ \6/ \5/ \4/ -> \7/ ...
Figure 3) The faces and sides of an octahedron.

Where each line represents the faces of a pyramid that wrap around from 3 to 0, or vise versa, and 4 to 7, respectively. Each face is a triangle of tiles indexed as shown below:

        /0\      /0\      /0\      /0\  
      /1\2/3\  /1\2/3\  /1\2/3\  /1\2/3\
      \3/2\1/  \3/2\1/  \3/2\1/  \3/2\1/
        \0/      \0/      \0/      \0/  
Figure 4) Face indices relative to side.

This inversion of the lower faces can also be seen as bending the array in half, or a rotation of pi, allowing the movement algorithm to simply invert directions for the lower faces.

## Tile Movement
The algorithm for tile movement is a series of conditional checks for special cases of face change and a base case for when the movement remains inside the same face.
### Base Case
Basic directional movement within a face is done by operating on index locations.  There are algorithms for handling movement based on three axes, labeled X, Y, and Z, in both positive and negative directions. The axes are named clockwise starting with Positive X being perpendicular to the equator, which is roughly north. The axes of movement are as shown below:

             /  \             
          /  \+X/  \          
       /  \-Z/##\+Y/  \       
    /  \-Y/  \-X/  \+Z/  \    
Figure 5a) All possible upper face movement.

    \  /-Z\  /+X\  /+Y\  /    
       \  /-Y\##/+Z\  /       
          \  /-X\  /          
             \  /             
Figure 5b) All possible lower face movement.

The figures above show movement not only for both upper and lower faces, but also the difference in movement between triangles based on their orientation, either +X is a vertex or a side. This is important since not every triangle along the axis is equidistant; some are twice as far as others. Since the centroid's distance from any vertex is twice that to the midpoint of any side, the distance traveled over a vertex is double that over a side. Here we see single distance moves for both halves

             /  \             
          /  \  /  \          
       /  \-Z/##\+Y/  \       
    /  \  /  \-X/  \  /  \    
Figure 6a) Upper face single moves.

    \  /  \  /+X\  /  \  /    
       \  /-Y\##/+Z\  /       
          \  /  \  /          
             \  /             
Figure 6b) Lower face single moves.

And the following shows double distance moves for both halves.

             /  \             
          /  \+X/  \          
       /  \  /##\  /  \       
    /  \-Y/  \  /  \+Z/  \    
Figure 7a) Upper face double moves.

    \  /-Z\  /  \  /+Y\  /    
       \  /  \##/  \  /       
          \  /-X\  /          
             \  /             
Figure 7b) Lower face double moves.

### Changing Face
If movement is not within the same face, the new face has to be calculated as well.  Since there is always a fixed number of faces in an octahedron, this is hard coded in match statements. There are four different kinds of face changes: pole movement,equatorial corners, equatorial crossing, side crossing, the former two of which are around points of distortion.

#### Pole Movement
Changing faces on the poles in simple and straightforward, there are three cases: positive X, positive Y, and negative Z. Every face change from one pole will land on another pole and the only calculations required are which face to change to. Each pole is one of the points of distortion. The following shows the defined movement for north pole; for the southern pole, swap the top face with its corresponding lower face.

      \18/
      / 0\
Figure 8a) Moving across the top pole along positive X.

It is important to note that moving positive X for both 0 and 18 will lead to the other. This is the only case where this will happen.


      \  /+Y
      /##\
Figure 8b) Moving around the top pol along positive Y.

This is the equivalent to rotating around the pole, where 1 -> 2, 2 -> 3, and 3 -> 0.

    -Z\  /
      /##\
Figure 8a) Moving around the top pole along negative Z.

This is the equivalent to rotating around the pole but in the opposite direction.
#### Equatorial Corners
The other points of distortion on the octahedron are the bottom corners of the face. Each of these again form squares around the vertex, There is only one special case per corner direction that has to be considered, the other directions are either covered by the next two cases or do not change face. The two cases checked are if you are moving across the vertex along the same axis. The equivalent face change is shown below for an octahedron of size 1.

    /  \/##\/  \/  \
    \-Y/\  /\+Z/\  /
Figure 9a) Corner movements for upper face.

    /  \/-Z\/  \/+Y\
    \  /\  /\##/\  /
Figure 9b)Corner movements for lower face.

Note that all movement for single tile faces are corners.

#### Equatorial Crossing
When moving from a top face to a bottom face and it is not a corner, the calculation simply finds the inverse column value of the bottom row for moving along the X axis.  If it is moving along the Y or Z axis it simply adds or subtracts 2 appropriately as shown below. Note that none of these movements cross a point of distortion.

             /  \             
          /  \  /  \          
       /  \  /##\  /  \       
       \-Y/  \-X/  \+Z/       
          \  /  \  /          
             \  /             
Figure 10a) Crossing equator from upper face.

             /  \             
          /  \  /  \          
       /-Z\  /+X\  /+Y\       
       \  /  \##/  \  /       
          \  /  \  /          
             \  /             
Figure 10b) Crossing equator from lower face.

#### Side Crossing
The last face change is over the edge of one triangle to the other. This can happen for both side and vertex movements when moving perpendicular to the face edge. That is, whether this movement is across a side or vertex of the triangle. Below shows this works for the upper face.

         /  \    ->    /+X\   
      /  \  /##\ -> /+Y\  /  \
Figure 11a) Moving across upper face edge over side.

         /+X\    <-    /  \   
      /  \  /-Z\ <- /##\  /  \
Figure 11b) Moving across upper face edge over side, other direction.

         /  \    ->    /  \   
      /  \##/  \ -> /  \+Y/  \
Figure 11c) Moving across upper face edge over vertex.

         /  \    <-    /  \   
      /  \-Z/  \ <- /  \##/  \
Figure 11d) Moving across upper face edge over vertex, other direction.
## Code Testing
There are three tests that each check movement in every direction for all tiles of octahedrons of size 1, 2 and 3, respectively.  With the algorithms the use of squares, multiplication, and addition, the tests give confidence in the algorithms correctness for anything larger.
## Future Plans
While the code feels more or less complete, the fact remains that there is no function that would allow one to travel straight along one axis of movement and return to the exact same tile; one of the goals of the program.  I believe this requires changing of direction of movement when changing face. This would either require a walking function to detect face and apply direction changes, such as positive X to negative X when crossing over the pole, on face changes or to have the movement function return the new direction of movement. The latter would be easiest to implement and chain if the function takes and returns a tuple of direction and tile id.
