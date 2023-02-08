function numIslands(image: string[][]): number {
    let count = 0;
    const h = image.length, w = image[0].length;
    
    function processSpot(i, j) {
        if(
           image[i]?.[j] == "1"
          ) {
            image[i][j] = "0";
            processSpot(i - 1, j);
            processSpot(i + 1, j);
            processSpot(i, j - 1);
            processSpot(i, j + 1);
        }
    }
    
    for (let i = 0; i < h; i++) {
        for (let j = 0; j < w; j++) {
            if(image[i][j] == "1") {
                processSpot(i, j);
                count += 1;
            }
        }
    }
    
    return count;
};

