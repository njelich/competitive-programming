function floodFill(image: number[][], sr: number, sc: number, color: number): number[][] {
    if(image[sr][sc] == color) return image;
    let curr = [[sr, sc]];
    const h = image.length, w = image[0].length;
    while (curr.length) {
        let next = curr.flatMap(
            (n) => [[n[0] - 1, n[1]], [n[0] + 1, n[1]], [n[0], n[1] - 1], [n[0], n[1] + 1]]
                .filter((sub) => sub[0] >= 0 && sub[0] < h &&
                        sub[1] >= 0 && sub[1] < w &&
                        image[sub[0]][sub[1]] == image[n[0]][n[1]]));
        curr.forEach((c) => image[c[0]][c[1]] = color);
        curr = next;
    }
    return image;
};