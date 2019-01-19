package year2018.day7

import java.io.File
import java.lang.RuntimeException

class Node(val name: String) {
    var nexts: MutableList<Node> = mutableListOf()
    private var incoming: Int = 0

    fun addNext(next: Node) {
        nexts.add(next)
        next.incoming += 1
    }

    fun decrement() {
        incoming--
    }

    fun isReady() = incoming == 0

    override fun toString(): String {
        return "Node(name: $name, incoming: $incoming)"
    }
}

class Worker {
    var node: Node? = null
    private var remainingLoad: Int? = null

    fun start(node: Node, baseSeconds: Int) {
        this.node = node
        remainingLoad = node.name.first().toUpperCase() - 'A' + 1 + baseSeconds
    }

    fun decrement() {
        if (remainingLoad != null) {
            remainingLoad = remainingLoad!! - 1
        }
    }

    fun isDone() = remainingLoad == 0
}

// part 1
fun topsort(path: String): String {
    val nodes = parseFile(path).toMutableList()
    val order = StringBuffer()
    while (nodes.isNotEmpty()) {
        val node = nodes.sortedBy { it.name }.first()
        nodes.remove(node)
        order.append(node.name)
        for (next in node.nexts) {
            next.decrement()
            if (next.isReady()) {
                nodes.add(next)
            }
        }
    }

    return order.toString()
}

// part 2
class Solution2(path: String, private val workerNum: Int,
                private val baseSeconds: Int = 0) {
    private val nodes = parseFile(path).toMutableList()
    private val order = StringBuffer()
    private val busyWorkers = mutableListOf<Worker>()
    private val idleWorkers = mutableListOf<Worker>()

    fun topsort(): Int {
        for (i in 0 until workerNum) {
            idleWorkers.add(Worker())
        }
        var seconds = 0
        while (true) {
            seconds++
            assignWork()
            tick()
//        println("tick, order: $order")
            if (nodes.isEmpty() && busyWorkers.isEmpty()) {
                break
            }
        }

        return seconds
    }

    // time passes by 1 second
    private fun tick() {
        for (i in 0 until busyWorkers.size) {
            val worker = busyWorkers.removeAt(0)
            worker.decrement()
            if (worker.isDone()) {
//            println("worker is done: ${worker.node!!.name}")
                order.append(worker.node!!.name.first())
                for (next in worker.node!!.nexts) {
                    next.decrement()
                    if (next.isReady()) {
//                    println("adding next ${next.name}")
                        nodes.add(next)
                    }
                }
                idleWorkers.add(worker)
            } else {
                // add it back
                busyWorkers.add(worker)
            }
        }
//    println("after tick, nodes: $nodes, idleWorkers: $idleWorkers, busyWorkers: $busyWorkers")
    }

    private fun assignWork() {
        nodes.sortBy { it.name }
        while (nodes.isNotEmpty() && idleWorkers.isNotEmpty()) {
            val node = nodes.removeAt(0)
            val worker = idleWorkers.removeAt(0)
            worker.start(node, baseSeconds)
            busyWorkers.add(worker)
        }
//    println("after assignWork, nodes: $nodes, idleWorkers: $idleWorkers, busyWorkers: $busyWorkers")
    }
}

// Return a list of nodes with no incoming edges
fun parseFile(path: String): List<Node> {
   val nodes: MutableMap<String, Node> = mutableMapOf()
    File(path).forEachLine { line ->
        val pair = parseLine(line)
        val name1 = pair.first
        val name2 = pair.second
        val node1 = nodes.getOrDefault(name1, Node(name1))
        val node2 = nodes.getOrDefault(name2, Node(name2))
        node1.addNext(node2)
        nodes[name1] = node1
        nodes[name2] = node2
    }

    return nodes.values.filter { it.isReady() }
}

// Step A must be finished before step L can begin.
fun parseLine(line: String): Pair<String, String> {
    val regex = """Step (\S+) must be finished before step (\S+) can begin.""".toRegex()
    if (regex.matches(line)) {
        val (_, name1, name2) = regex.find(line)!!.groupValues
        return Pair(name1, name2)
    } else {
        throw RuntimeException("Invalid input line: $line")
    }
}
