# Raytracer-In-Rust
My implementation of a basic ray-tracer inspired by the book "Ray Tracing in One Weekend" written in Rust.

## Final Render
![finalRenderImage]

[finalRenderImage]: https://i.imgur.com/tUKl22R.png

# The Process

In this section I will detail with images the process I took to make the ray tracer. Including some cool looking renders due to bugs and errors along the way.

### Test Image
![image]

[image]: https://i.imgur.com/XQtc9Qm.png

The above image is more of an test image to check that the file format works, producing the image we expect to see.

### Ray colour gradient
![image2]

[image2]: https://i.imgur.com/nkT7Aja.png

The above image shows the start of rays being implemented along with their own colours. Where we blend from blue to a white gradient depending on the rays Y co-ordinate.

### Initial Sphere and hit registration
![image3]

[image3]: https://i.imgur.com/OZ3YfDa.png

This image shows how we added a simple sphere in a fixed location in space where if a ray hit the sphere we would colour the pixel red.

### Colouring the sphere with surface normals
![image4]

[image4]: https://i.imgur.com/k0CvVfk.png

Here we added shading with Surface normals which produces the above coloured effect on the sphere.

### Diffuse lighting and materials to create shadows
![image5]

[image5]: https://i.imgur.com/06IN5HA.png

In the above image we have added Diffuse materials where rays are scattered and child rays are created. This results in shadows. We also added anti-aliasing to smooth out rough edge's. 

### Different material types
![image6]

[image6]: https://i.imgur.com/eFDvESL.png

Next we moved onto implementing different material types. In the image above we have 3 different spheres with different fuzz amounts resulting in reflections of the other spheres, just not perfect ones. They also have a albedo variable which decides what colour the material is.

### Start of glass material type
![image7]

[image7]: https://i.imgur.com/JbrC6Pg.png

Next we added another material type of glass or Dielectric. This means we had to implement refraction and total internal reflection creating the effect of light being bent by the glass sphere.

### Creating a more realistic glass sphere
![image8]

[image8]: https://i.imgur.com/JZk9z7r.png

To make the glass look even more realistic we use an approximation by Christophe Schlick for reflectivity that varies with angle allowing us to create a more realistic looking glass sphere.

### Moveable Camera
![image9]

[image9]: https://i.imgur.com/QLXBfFZ.png

Now that we have implemented several different materials the next step was to work on a moveable camera. This would allow us to look at the scene for any position in 3D space that we desire. The above image is another render of the same scene but from a far away shot. And the below render is of the same angle but much smaller field of view giving it the effect of being zoomed in.

![image10]

[image10]: https://i.imgur.com/8qKTJ24.png

### Adding a Depth of Field Effect
![image11]

[image11]: https://i.imgur.com/FSGscnO.png

To make the camera more realistic we added a depth of field effect. Simulating how the lens of a camera would affect our rendered image of the scene.

# Final Render

And finally we have the finished Product. This is a scene using all of the features we have talked about so far all in one scene. The image is rendered at 1200 pixels 800 pixels in a 3:2 ratio with 500 samples per pixel to create a fantastic looking image.

![finalRenderImage]

# Some cool unintended Renders

![unintended1]

[unintended1]: https://i.imgur.com/nNGCC2R.png

![unintended2]

[unintended2]: https://i.imgur.com/G4eoujU.png

![unintended3]

[unintended3]: https://i.imgur.com/hQoPczR.png

![unintended4]

[unintended4]: https://i.imgur.com/dxnunZ8.png

![unintended5]

[unintended5]: https://i.imgur.com/NWl3kGj.png