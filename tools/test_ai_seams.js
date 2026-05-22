"use strict";
const http = require("http");

const testMesh = {
    vertices: [
        { x: -1, y: -1, z:  1 }, { x:  1, y: -1, z:  1 }, { x:  1, y:  1, z:  1 }, { x: -1, y:  1, z:  1 },
        { x: -1, y: -1, z: -1 }, { x:  1, y: -1, z: -1 }, { x:  1, y:  1, z: -1 }, { x: -1, y:  1, z: -1 }
    ],
    indices: [
        0, 1, 2, 0, 2, 3, 1, 5, 6, 1, 6, 2, 5, 4, 7, 5, 7, 6,
        4, 0, 3, 4, 3, 7, 3, 2, 6, 3, 6, 7, 4, 5, 1, 4, 1, 0
    ]
};

function requestSeams() {
    const postData = JSON.stringify(testMesh);
    const options = {
        hostname: "127.0.0.1",
        port: 8000,
        path: "/api/seams/predict?curvature_deg=45",
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "Content-Length": Buffer.byteLength(postData)
        }
    };

    const req = http.request(options, (res) => {
        let data = "";
        res.on("data", (chunk) => { data += chunk; });
        res.on("end", () => {
            if (res.statusCode === 200) {
                console.log("[SUCCESS] AI Response OK:");
                console.log(JSON.stringify(JSON.parse(data), null, 2));
            } else {
                console.error("[ERROR] Server Error:", res.statusCode, "-", data);
            }
        });
    });

    req.on("error", (e) => {
        console.error("[REQUEST ERROR] Connection failed:", e.message);
    });

    req.write(postData);
    req.end();
}

console.log("Testing AI Seams API...");
requestSeams();
