# What is Octo Sphere?
The purpose of this code is to dynamically tile a sphere using an octahedron whose faces are subdivided into equilateral triangles of equal area in such a way that allows adjacent tile movement that can circumnavigate the sphere and end up at the origin if chained. This document contains an induction on the motivation behind the project, detailed analysis of space and movement within it, implemented unit tests, and plans for further improvement.
### Table of Contents
1. [Program Motivation](#Program-Motivation)
2. [The Octahedron](#The-Octahedron)
3. [Local Movement](#Local-Movement)
    - [Base Case](#Base-Case)
    - [Changing Face](#Changing-Face)
       - [Polar Corners](#Polar-Corners)
       - [Equatorial Corners](#Equatorial-Corners)
       - [Equatorial Crossing](#Equatorial-Crossing)
       - [Side Crossing](#Side-Crossing)
4. [Walking the Octahedron](#Walking-the-Octahedron)
    - [Chaining Steps](#Chaining-Steps)
    - [Rotating Orientation](#Rotating-Orientation)
       - [Changing Face Along and to X](#Changing-Face-Along-and-to-X)
       - [Flipping Between Y and Z](#Flipping-Between-Y-and-Z)
5. [Code Testing](#Code-Testing)
    - [Step Tests](#Step-Tests)
    - [Walk Tests](#Walk-Tests)
6. [Future Plans](#Future-Plans)
## 1. Program Motivation
The classic approach to tiling spheres is done using square tiles and mapped onto a cylinder as this allows east-west wrapping. This has been the common approach for decades until recently. More modern spherical tilings have moved from square tiles to hexagons, as it allows more degrees of equidistant movement. The problem with tiling a sphere with equal sized hexagons is that it simply can't be done. Most programs tile a cube with hexagons, leaving the corners as pentagons, and map this to a sphere. What if one wants all the tiles the same shape and still wants to retain the hexagonal axes? Consider a hexagon as the following six equilateral triangles:

    /1\2/3\
    \5/6\7/
Figure 1.1) A hexagon made from equilateral triangles.

Here we can see that moving through the triangles from 1 to 7 lies on the same axis. Similarly, 5 and 3 also sit on the same, albeit different, axis. Additionally, the numbers above and below one another are also along a third axis, each column being parallel to the others. All three of these axes are the same as those that would be in any hexagonal tiling. Furthermore, this hexagon can easily be made into a triangle by adding a few more tiles, as shown in the next figure.

        /0\
      /1\2/3\
    /4\5/6\7/8\
Figure 1.1) A equilateral triangle subdivided into equilateral triangles.

This triangle can then form one of the eight faces of an octahedron. While this isn't a perfect mapping, it has some niceties in regards to local movement along the axes. Each corner vertex loses two triangles in the hexagon that would form around it (note that this actually forms a square at the points of distortion) but since they are the two triangles that don't correlate with the axial movements, they can be safely disregarded.
## 2. The Octahedron
The data structure of this polyhedron is simply a vector of tiles, with each one having a unique id calculated from which face in lies on, the number of tiles on each face, and index position on the face it sits. The construction takes a size that defines the face length in the number of tile sides, where a value of 1 would have faces being a single triangle. The movement algorithm constructs all eight faces the same but pretends that the last four are flipped. This creates pairs of faces as seen here:

    /0\ /1\ /2\ /3\ → /0\
    \7/ \6/ \5/ \4/ → \7/ ...
Figure 2.1) The faces and sides of an octahedron.

Where each row represents the faces of a pyramid that wrap around from 3 to 0, or vise versa, and 4 to 7, respectively. Each face is a triangle of tiles indexed as shown below:

        /0\      /0\      /0\      /0\  
      /1\2/3\  /1\2/3\  /1\2/3\  /1\2/3\
      \3/2\1/  \3/2\1/  \3/2\1/  \3/2\1/
        \0/      \0/      \0/      \0/  
Figure 2.1) Face indices relative to side.

This inversion of the lower faces can also be seen as bending the array in half, or a rotation one radian, allowing the movement algorithm to simply invert directions for the lower faces.

The main source file contains a simple display similar to the above figure using a size variable but displays unique tile ids. The current size value is set to five but it can be raise or lowered. Note that if you do *increase* the size variable, be sure to change the display function by increasing the spaces and :4 in the println macros to a larger value for ideal aesthetics.

## 3. Local Movement
The algorithm for tile movement is a series of conditional checks for special cases for face changes and a base case for when the movement remains inside the same face.
### Base Case
Basic directional movement within a face is done by operating on index locations. There are checks to handle movement based on three axes, labeled X, Y, and Z, in both positive and negative directions. The axes are named clockwise starting with Positive X being perpendicular to the equator, which is roughly north. The axes of movement are as shown below:

             /  \             
          /  \+X/  \          
       /  \-Z/##\+Y/  \       
    /  \-Y/  \-X/  \+Z/  \    
---
    \  /-Z\  /+X\  /+Y\  /    
       \  /-Y\##/+Z\  /       
          \  /-X\  /          
             \  /             
Figure 3.1) All possible upper and lower face movements.

The figures above show movement not only for both upper and lower faces, but also the difference in movement between triangles based on their orientation, either +X crosses over a vertex or an edge. This is important since not every triangle along the axis is equidistant; some are twice as far as others. Since the centroid's distance from any vertex is twice that to the midpoint of any side, the distance traveled over a vertex is double that over an edge. THe following figure shows single distance moves for both halves.

             /  \             
          /  \  /  \          
       /  \-Z/##\+Y/  \       
    /  \  /  \-X/  \  /  \    
---
    \  /  \  /+X\  /  \  /    
       \  /-Y\##/+Z\  /       
          \  /  \  /          
             \  /             
Figure 3.2) Upper and lower face single moves.

And the next shows double distance moves for both halves.

             /  \             
          /  \+X/  \          
       /  \  /##\  /  \       
    /  \-Y/  \  /  \+Z/  \    
---
    \  /-Z\  /  \  /+Y\  /    
       \  /  \##/  \  /       
          \  /-X\  /          
             \  /             
Figure 3.3) Upper and lower face double moves.

### Changing Face
If movement is not within the same face, the new face has to be calculated as well. Since there is always a fixed number of faces on an octahedron, this is hard coded in match statements. There are four different kinds of face changes: polar corners,equatorial corners, equatorial crossing, and side crossing, the former two of which are around points of distortion.

#### Polar Corners
Changing faces around the polar corner in simple and straightforward, there are three cases: positive X, positive Y, and negative Z. Every face change from one pole will land on another pole and the only calculations required are which face to change to. The following shows the defined movement for north pole.

      \18/
      / 0\
Figure 3.4) Moving across the top pole along positive X.

It is important to note that moving positive X for both 0 and 18 will lead to the other. This is the only case where this will happen.


      \  /+Y
      /##\
Figure 3.5) Moving around the top pole along positive Y.

This is the equivalent to rotating around the pole, where 1 → 2, 2 → 3, and 3 → 0.

    -Z\  /
      /##\
Figure 3.6) Moving around the top pole along negative Z.

This is the equivalent to rotating around the pole but in the opposite direction.
#### Equatorial Corners
The other points of distortion on the octahedron are the bottom corners of the face. Each of these again form squares around the vertex. There is only one special case per corner direction that has to be considered, the other directions are either covered by the next two cases or do not change face. The two cases checked are if you are moving across the vertex along the same axis. The equivalent face change is shown below for an octahedron of size 1.

    /  \/##\/  \/  \
    \-Y/\  /\+Z/\  /
---
    /  \/-Z\/  \/+Y\
    \  /\  /\##/\  /
Figure 3.7)Corner movements for upper and lower faces. Note that all movement for single tile faces are corners.

#### Equatorial Crossing
When moving from a top face to a bottom face and it is not a corner, the calculation simply finds the inverse column value of the bottom row for moving along the X axis. If it is moving along the Y or Z axis it simply adds or subtracts 2 appropriately as shown below. Note that none of these movements cross a point of distortion.

             /  \             
          /  \  /  \          
       /  \  /##\  /  \       
       \-Y/  \-X/  \+Z/       
          \  /  \  /          
             \  /             
Figure 3.8) Crossing equator from upper face.

             /  \             
          /  \  /  \          
       /-Z\  /+X\  /+Y\       
       \  /  \##/  \  /       
          \  /  \  /          
             \  /             
Figure 3.9) Crossing equator from lower face.

#### Side Crossing
The last face change is over the edge of one triangle to the other. This can happen for both side and vertex movements when moving perpendicular to the face edge. That is, whether this movement is across a side or vertex of the triangle. Below shows this works for the upper face.

         /  \    →    /+X\   
      /  \  /##\ → /+Y\  /  \
---
         /+X\    ←    /  \   
      /  \  /-Z\ ← /##\  /  \
---
         /  \    →    /  \   
      /  \##/  \ → /  \+Y/  \
---
         /  \    ←    /  \   
      /  \-Z/  \ ← /  \##/  \
Figure 3.10) Moving across the upper face side. Top: Movement across the +Y side over a vertex. Second: Movement across the -Z side over a vertex. Third: Movement across the +Y side over a edge. Second: Movement across the -Z side over a edge.
## 4. Walking The Octahedron
Thinking of a walking, driving or any other surface movement, on a sphere, if something were to just pick a direction on a sphere and travel it continuously, the object should return to it's original location, facing the same direction it started. That is walking is repeatedly taking step after step so the step function continuously called on it's own output should represent walking the sphere.
### Chaining Steps
The original creation of the step function took and tiled id and direction but only returned a new tile identifier so in order to facilitate direction changes and chain the input and output were both changed to a tuple of tile identifier and direction. The input represent choosing a tile and then stepping in a certain direction while the output is the resulting tile identifier and the orientation from which one arrived. This allows the step to be called again and again and again. The base case, that is when the step remains in the same face, is the same direction that was called. Nicely enough, traveling across the vertex of an equatorial corner doesn't change the orientation either. The edge cases are where the direction that the step enters a triangle is not the same as that from which it left.
### Rotating Orientation
The crux of the return orientation is that your new axis is that from which you cross into the triangle on. To help understand this we can imagine the hexagon in which we are traveling across. First we consider a point from which a hexagon forms around as shown below.

         / ↘\._ _ _ _./ ↙\   
      /  \ ↗/ ↑\   / ↑\ ↖/  \
---
         / ↘\.↙/ ↖\  /
      /  \ ↗/ ↑\ ↑/   
Figure 4.1) All directions to the same point on an edge. Top: Viewing the point on separate faces, note that both dots are the same point. Bottom: Viewing the point as the center of a single hexagon.

Each arrow in the figure above shows the direction that leads to that point. The bottom hexagon is formed by rotating the left triangle one third of a radian. We can see the first two top rows are polar corners. If you were to take those steps around that point the resulting orientation would be the opposite direction on the arriving tile.

The other set of cases happen when you travel across a side, as seen in the following figure. Here we can see than we simply switch the Y and Z axis.

         / ↗\. - - - ./ ↖\   
      /  \  / ↗\ - / ↖\  /  \
---
         / ↗\.↖/  \  /
      /  \  / ↗\ ↖/   
Figure 4.2) Direction change across the side. Top: Viewing the sides on separate faces, note that both dots are the same point. Bottom: Viewing the sides as part of a single hexagon.
#### Changing Face Along and to X
Orientation change around the X axis can be both the simplest and the most complicated. Starting with the simplest, movement directly across the poles is simply flipping from positive X to negative X or vise versa.

      \ ↑/
      / ↑\
---
      \ ↓/
      / ↓\
Figure 4.3) Moving across the pole along positive and negative X or vise versa, depending on the pole.

But if one moves up across a side, it depends on which side one is on. The figure below shows how the orientation changes when changing face.

         /  \ _ _ _ _ / ↗\   
      /  \  / ↑\   /  \  /  \
---
         / ↖\ _ _ _ _ /  \   
      /  \  /  \   / ↑\  /  \
Figure 4.4) Positive X on a side. Top: Moving across the +Y side. Bottom: Moving across the -Z side.

But if we leave the X axis, how do we return? There are two cases depending on the side. When traveling across a non-corner point, similar but opposite to how one left the X axis,

         / ↘\ _ _ _ _ /  \   
      /  \  /  \   / ↓\  /  \
---
         /  \ _ _ _ _ / ↙\   
      /  \ ↓/  \   /  \  /  \
Figure 4.5) Returning to negative X on a side. Top: Moving across the +Y side. Bottom: Moving across the -Z side.

#### Flipping Between Y and Z
The last of the orientation changes are a simple flipping of Y and Z axes when traveling across the vertex or edge in the same as the face side being crossed as the next figure shows.

         / ↗\ - - - - / ↘\   
      /  \  /  \   /  \  /  \
---
         /  \ _ _ _ _ /  \   
      /  \ ↗/  \   /  \ ↘/  \
Figure 4.7) Top: Flat +Y to +Z. Bottom: Point +Y to +Z

## 5. Code Testing
I have always liked the idea of test driven development so before I set about writing any code, I wrote a series of black box unit test. I did not write all the test at once but rather as a series of steps. First, of course, with the step test that tested each tile on variable sizes and then wrote the code for it, running the test to make sure my conditions matched with incremental changes, one condition at a time, and I did likewise for the walking test after I had finished the step function with direction.
### Step Tests
There are three tests that each check movement in every direction for all tiles of octahedrons of size 1, 2 and 3, respectively. They simply call the step function an compare it to an expected result. The hard coded comparisons were derived from lots for drawing of equilateral triangles. Since the algorithms uses values that are squared, doubled, and incremented, the tests should give confidence in the algorithms
correctness for anything larger.
### Walk Tests
The second series of tests make sure that one can circumnavigate the octahedron. This is done by a simple while loop that stops when it returns to the start position and checks that last return value was the same as the start value, both for tile identifier and direction. Admittedly, having a potentially infinite loop for a quiet test is not the smartest idea but, since I got really got at drawing equilateral triangles, it worked. At some point I should give it a guard that panics after a number of moves.

## 6. Future Plans
Did I mention spheres? There really is nothing spherical about it at the moment. The next step would be to correct this and implement a spherical coordinate mapping of each tile centroid. This should be fairly easy since each face covers a half radian arc of both the inclination and azimuth.
