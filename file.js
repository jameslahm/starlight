function Node(left, right) {
    this.left = left;
    this.right = right;
}
let nNodes = 0;
function makeTree(depth) {

    nNodes += 1;
    if (depth == 0) {

        return new Node(undefined, undefined);
    }
    let n1 = makeTree(depth - 1);
    let n2 = makeTree(depth - 1);
    return new Node(n1, n2);
}

let tree = makeTree(1);

print("Created ", nNodes, " nodes");