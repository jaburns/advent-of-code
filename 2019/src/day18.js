const fs = require('fs');

const maze = fs.readFileSync('data/day18.txt', 'utf8').split('\n').map(x => x.trim().split('').map(x => ({ dist: 0, char: x })));

const logMaze = () => {
    console.log(maze.map(x => x.map(x => x.char).join('')).join('\n'));
};

const findKey = k => {
    for (let y = 0; y < maze.length; ++y) {
        for (let x = 0; x < maze[y].length; ++x) {
            if (maze[y][x].char === k) return [x, y];
        }
    }
    throw new Error();
}

const clearMazeDistances = () => {
    for (let y = 0; y < maze.length; ++y) {
        for (let x = 0; x < maze[y].length; ++x) {
            maze[y][x].dist = 0;
        }
    }
}

const findDistanceFromKeyToKey = (a, b) => {
    const [ax, ay] = findKey(a);
    const [bx, by] = findKey(b);

    clearMazeDistances();

    let frontier = [ [ax, ay] ];
    let curDist = 1;
    let foundIt = false;

    const freePath = (x, y) => {
        if (maze[y][x].char !== '#' && maze[y][x].dist === 0) return true;
        return false;
    };

    outer: while (frontier.length > 0) {
        for (let i = 0; i < frontier.length; i++) {
            maze[frontier[i][1]][frontier[i][0]].dist = curDist;
            if (frontier[i][0] === bx && frontier[i][1] === by) {
                foundIt = true;
                break outer;
            }
        }

        let newFrontier = [];

        for (let i = 0; i < frontier.length; i++) {
            let [x, y] = frontier[i];
            if (freePath(x + 1, y)) newFrontier.push([x + 1, y]);
            if (freePath(x - 1, y)) newFrontier.push([x - 1, y]);
            if (freePath(x, y + 1)) newFrontier.push([x, y + 1]);
            if (freePath(x, y - 1)) newFrontier.push([x, y - 1]);
        }

        curDist++;
        frontier = newFrontier;
    }

    if (!foundIt) {
        return [ 0 ];
    }

    let doors = [];
    let wx = bx, wy = by, wd = curDist;

    const tryWalk = (x, y) => {
        if (maze[y][x].dist > 0 && maze[y][x].dist < wd) {
            wx = x;
            wy = y;
            wd = maze[y][x].dist;
            if (maze[y][x].char.charCodeAt(0) >= 65 && maze[y][x].char.charCodeAt(0) <= 90) {
                doors.push(maze[y][x].char);
            }
            return true;
        }
        return false;
    };

    while (wd !== 1) {
        if (tryWalk(wx + 1, wy)) continue;
        if (tryWalk(wx - 1, wy)) continue;
        if (tryWalk(wx, wy + 1)) continue;
        if (tryWalk(wx, wy - 1)) continue;
    }

    return [curDist - 1, doors];
}

const canUnlock = (keys, doors) => {
    doors = doors.map(x => x.toLowerCase());
    for (let d = 0; d < doors.length; ++d) {
        if (keys.indexOf(doors[d]) < 0) return false;
    }
    return true;
}

const ALL_KEYS = 'abcdefghijklmnopqrstuvwxyz'.split('');

const part1 = () => {
    console.log("Part 1...");

    const table = {};

    ALL_KEYS.concat(['@']).forEach(a => {
        ALL_KEYS.forEach(b => {
            table[a+b] = findDistanceFromKeyToKey(a, b);
        });
    });

    let worlds = [{
        stepsTaken: 0,
        collected: ['@']
    }];

    let minSteps = Infinity;

    const visitedHash = new Set();

    while (worlds.length > 0) {
        worlds.sort((a, b) => a.stepsTaken - b.stepsTaken );
        const world = worlds.shift();
        const curKey = world.collected[world.collected.length-1];

        const stateHash = world.collected.slice().sort().join('') + ':' + curKey;
        if (visitedHash.has(stateHash)) continue;
        visitedHash.add(stateHash);

        ALL_KEYS.forEach(k => {
            if (k === curKey || world.collected.indexOf(k) >= 0) return;

            const [ dist, doors ] = table[curKey + k];

            if (canUnlock(world.collected, doors)) {
                let newWorld = {
                    stepsTaken: world.stepsTaken + dist,
                    collected: world.collected.concat([ k ]),
                };

                if (newWorld.collected.length === 27) {
                    if (newWorld.stepsTaken < minSteps) {
                        //console.log(newWorld);
                        minSteps = newWorld.stepsTaken;
                    }
                } else {
                    worlds.push(newWorld);
                }
            }
        });
    }

    console.log(minSteps);
};

const mutateMazeForPart2 = () => {
    maze[39][39].char = '$';
    maze[41][39].char = '%';
    maze[39][41].char = '^';
    maze[41][41].char = '&';

    maze[40][40].char = '#';
    maze[41][40].char = '#';
    maze[39][40].char = '#';
    maze[40][41].char = '#';
    maze[40][39].char = '#';
};

const part2 = () => {
    console.log("Part 2...");

    mutateMazeForPart2();

    const table = {};
    const robotKeyTable = {};

    ALL_KEYS.concat(['$','%','^','&']).forEach(a => {
        ALL_KEYS.forEach(b => {
            let dist = findDistanceFromKeyToKey(a, b);
            if (dist[0] > 0) {
                table[a+b] = dist

                if (['$','%','^','&'].indexOf(a) >= 0) {
                    robotKeyTable[b] = a;
                }
            }
        });
    });

    let worlds = [{
        stepsTaken: 0,
        "last$": '$',
        "last%": '%',
        "last^": '^',
        "last&": '&',
        collected: ['$', '%', '^', '&'],
    }];

    let minSteps = Infinity;

    const visitedHash = new Set();

    while (worlds.length > 0) {
        worlds.sort((a, b) => a.stepsTaken - b.stepsTaken );
        const world = worlds.shift();

        const stateHash = world.collected.slice().sort().join('') + ':'
            + world['last$'] + world['last%'] + world['last^'] + world['last&'];
        if (visitedHash.has(stateHash)) continue;
        visitedHash.add(stateHash);

    //  console.log(worlds.length, " :: ", stateHash.length, " / 34");

        ['$','%','^','&'].forEach(robot => {
            const curKey = world['last'+robot];

            ALL_KEYS.forEach(k => {
                if (k === curKey || world.collected.indexOf(k) >= 0) return;
                if (!((curKey + k) in table)) return;

                const [ dist, doors ] = table[curKey + k];

                if (canUnlock(world.collected, doors)) {
                    let newWorld = {
                        stepsTaken: world.stepsTaken + dist,
                        collected: world.collected.concat([ k ]),
                    };

                    newWorld["last$"] = world["last$"];
                    newWorld["last%"] = world["last%"];
                    newWorld["last^"] = world["last^"];
                    newWorld["last&"] = world["last&"];
                    newWorld['last' + robot] = k;

                    if (newWorld.collected.length === 30) {
                        if (newWorld.stepsTaken < minSteps) {
                            minSteps = newWorld.stepsTaken;
                        }
                    } else {
                        worlds.push(newWorld);
                    }
                }
            });
        });
    }

    console.log(minSteps);
};

part2();
