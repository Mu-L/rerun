// Log a simple colored triangle.

#include <rerun.hpp>

#include <cmath>
#include <numeric>

int main() {
    auto rec = rerun::RecordingStream("rerun_example_mesh3d_simple");
    rec.connect("127.0.0.1:9876").throw_on_failure();

    rerun::Position3D vertex_positions[3] = {
        {0.0f, 0.0f, 0.0f},
        {1.0f, 0.0f, 0.0f},
        {0.0f, 1.0f, 0.0f},
    };
    rerun::Color vertex_colors[3] = {
        {255, 0, 0},
        {0, 255, 0},
        {0, 0, 255},
    };

    rec.log(
        "triangle",
        rerun::Mesh3D(vertex_positions)
            .with_vertex_normals({{0.0f, 0.0f, 1.0f}})
            .with_vertex_colors(vertex_colors)
    );
}
