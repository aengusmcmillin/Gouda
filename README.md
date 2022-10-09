# Gouda

Gouda Engine
by Cheddar Games

ECS based Game Engine

## Architecture

- Entry Point
- App Layer
- Window Layer
  - Inputs
  - Events
- Renderer
- Render API Abstraction


## Todo List

### 3d Renderer


Rendering and ECS
Have a MaterialLibrary, and a MeshLibrary. 
A Mesh is a set of verts and indices to draw an indexed triangle list
A Material pairs a shader with a set of uniforms. Also maybe a texture.
A Shader would have a spec for expected vertex layout, as well as for the expected uniforms, to do validation on compatible materials and meshes

The Scene is passed renderables, which reference a material name, a mesh name, and a transform. This would be with 'Submit3D' commands.
For batch rendering, for each material name, that is mapped to a map from mesh names to lists of transforms. 
It would iterate through, load and bind the material, and then load and bind each model, binding and rendering for each transform. This would happen with a 'Draw' command.
Could also have a following pass on any 'Submit2D' objects, which would be all a single shader, single quad

So:
- Material 1
-- Model 1
--- Instance A
--- Instance B
-- Model 2
--- Instance C
- Material 2
-- Model 3
--- Instance D
