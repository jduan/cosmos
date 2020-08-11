package jduan

import org.eclipse.jgit.api.Git
import org.eclipse.jgit.lib.Repository
import org.eclipse.jgit.revwalk.RevCommit
import org.eclipse.jgit.revwalk.RevTree
import org.eclipse.jgit.revwalk.RevWalk
import org.eclipse.jgit.treewalk.AbstractTreeIterator
import org.eclipse.jgit.treewalk.CanonicalTreeParser
import java.io.File

class JgitExample(val path: File) {
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

    // Find the most recent 3 people who touched a file.
    fun findRecentAuthors(filePath: String, mostRecent: Int = 3): Set<String> {
        val commits = git.log().addPath(filePath).call()
        val filteredEmails = mutableListOf<String>()
        val allEmails = mutableListOf<String>()
        for (commit in commits) {
            val email = commit.authorIdent.emailAddress
            if (allEmails.size < mostRecent)
                allEmails.add(email)
        }
        return if (filteredEmails.isNotEmpty()) {
            filteredEmails
        } else {
            // fallback in case we can't find any non-refactoring commits
            allEmails
        }.toSet()
    }

    fun getMailMap(): Map<String, String> {
        val emailMap = HashMap<String, String>()
        val nameMap = HashMap<String, String>()
        val mailmap = path.resolve(".mailmap")
        println("mailmap file: $mailmap")
        val email_regex = "(<.+?>)\\s+(<.+?>)".toRegex()
        val name_regex = "([\\w ]+)\\s+(<.+?>)".toRegex()
        mailmap.readLines().forEach { line ->
            println("line: $line")
            if (email_regex.matches(line)) {
                val matches = email_regex.findAll(line)
                matches.forEach {
                    emailMap[it.groupValues[2]] = it.groupValues[1]
                }
            }

            if (name_regex.matches(line)) {
                val matches = name_regex.findAll(line)
                matches.forEach {
                    nameMap[it.groupValues[1]] = it.groupValues[2]
                }
            }
        }
        println("emailMap: $emailMap")
        println("nameMap: $nameMap")

        return emailMap
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
    val path = File("/Users/jingjing_duan/repos2/pineapple")
    val sha = "8cf913ab86260f1123dd48b25458c5dc62e14a4c"
    val filePath = "frontend/arrive/tests/containers/MapContainer_spec.jsx"
    val git = JgitExample(path)
//    git.getDiffFilesInCommit(sha)
    val authors = git.findRecentAuthors(filePath, 10)
    println("authors: $authors")
    git.getMailMap()
}
