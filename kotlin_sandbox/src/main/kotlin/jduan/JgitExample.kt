package jduan

import org.eclipse.jgit.api.Git
import org.eclipse.jgit.lib.Repository
import org.eclipse.jgit.revwalk.RevCommit
import org.eclipse.jgit.revwalk.RevTree
import org.eclipse.jgit.revwalk.RevWalk
import org.eclipse.jgit.treewalk.AbstractTreeIterator
import org.eclipse.jgit.treewalk.CanonicalTreeParser
import java.io.File

class JgitExample(path: File) {
    private val git: Git = Git.open(path)

    // Show the number of files changed in a commit.
    // A git commit is a snapshot of the entire git repo. To get the list of files changed in a commit,
    // you need to diff 2 snapshots.
    fun getDiffFilesInCommit(sha: String) {
        val repo = git.repository
        val diffs = git.diff()
            .setOldTree(prepareTreeParser(repo, "${sha}^"))
            .setNewTree(prepareTreeParser(repo, sha))
            .call()

        println("Found: " + diffs.size + " differences")
    }

    private fun prepareTreeParser(repository: Repository, objectId: String): AbstractTreeIterator? {
        // from the commit we can build the tree which allows us to construct the TreeParser
        val walk = RevWalk(repository)
        val commit: RevCommit = walk.parseCommit(repository.resolve(objectId))
        val tree: RevTree = walk.parseTree(commit.tree.id)
        val treeParser = CanonicalTreeParser()
        val reader = repository.newObjectReader()
        treeParser.reset(reader, tree.id)
        reader.close()
        walk.close()

        return treeParser
    }
}

fun main() {
    val path = File("/Users/jingjing_duan/repos2/airbnb")
    val sha = "8cf913ab86260f1123dd48b25458c5dc62e14a4c"
    JgitExample(path).getDiffFilesInCommit(sha)
}
