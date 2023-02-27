// - TAD boundary file (HAP-1.boundary.bed)

// chr1    1070000 1080000
// chr1    1265000 1275000
// chr1    1355000 1365000
// chr1    1505000 1515000

// - Chromatin loop file (HAP-1.loops.bedpe)
// Note: each row in HAP-1.loops.bedpe is a loop that containing two anchors.
// The first three columns are anchor one and the next three columns are anchor two.

// #chr1   x1      x2      chr2    y1      y2
// # juicer_tools version 2.13.06
// chr10   14950000        14960000        chr10   15100000        15110000
// chr10   95690000        95700000        chr10   95840000        95850000

// - ATAC-Seq peak file: (HAP-1-ATAC-seq.peak.bed)
// chr1    181400  181560  .       0       .       0.537938        -1      -1      75
// chr1    268011  268080  .       0       .       0.321915        -1      -1      75

// TODO: Please compute how many ATAC-Seq peaks are located in a loop anchor and how many of the ATAC-Seq peaks are located in a TAD boundary.
// Describe how you compute the overlaps step by step.

mod interval_tree;

use crate::interval_tree::{COITree, IntervalNode};

use clap::Parser;
use dashmap::DashMap;
use rayon::prelude::*;
use std::{io::BufRead, path::PathBuf};

use std::io::Write;

type IntervalTree = COITree<String, usize>;
type Node = IntervalNode<String, usize>;
type ChrIntervalTree = DashMap<String, IntervalTree>;

#[derive(Parser, Debug)]
struct Args {
    /// Path of ATAC-Seq peak file
    #[arg(short, long)]
    atac: PathBuf,

    /// Path of boundary file
    #[arg(short, long)]
    boundary: PathBuf,

    /// Path of loop file
    #[arg(short, long)]
    loops: PathBuf,
}

fn query_loop(path: &PathBuf, trees: &ChrIntervalTree) {
    let file = std::fs::File::open(path).unwrap();
    let content = std::io::BufReader::new(file);

    let result_path = path.with_extension("overlap.bedpe");

    let mut result = std::fs::File::create(result_path).unwrap();

    let it = content
        .lines()
        .skip(2)
        .par_bridge()
        .map(|x| {
            let line = x.unwrap();

            let mut content = line.split('\t');
            let chr1 = content.next().unwrap().to_string();
            let start1 = content.next().unwrap().parse::<i32>().unwrap();
            let end1 = content.next().unwrap().parse::<i32>().unwrap();

            let chr2 = content.next().unwrap().to_string();
            let start2 = content.next().unwrap().parse::<i32>().unwrap();
            let end2 = content.next().unwrap().parse::<i32>().unwrap();

            let count1 = trees
                .get(&chr1)
                .map(|tree| tree.value().query_count(start1, end1))
                .unwrap_or(0);

            let count2 = trees
                .get(&chr2)
                .map(|tree| tree.value().query_count(start2, end2))
                .unwrap_or(0);

            format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                chr1, start1, end1, count1, chr2, start2, end2, count2
            )
        })
        .collect::<Vec<String>>();

    it.iter().for_each(|x| {
        result.write_all(x.as_bytes()).unwrap();
    });
}

fn query_boundary(path: &PathBuf, trees: &ChrIntervalTree) {
    // chr1    1070000 1080000
    // chr1    1265000 1275000

    let file = std::fs::File::open(path).unwrap();
    let content = std::io::BufReader::new(file);

    let result_path = path.with_extension("overlap.bed");

    let mut result = std::fs::File::create(result_path).unwrap();

    let it = content
        .lines()
        .par_bridge()
        .map(|x| {
            let line = x.unwrap();

            let mut content = line.split('\t');

            let chr = content.next().unwrap().to_string();
            let start = content.next().unwrap().parse::<i32>().unwrap();
            let end = content.next().unwrap().parse::<i32>().unwrap();
            let count = trees
                .get(&chr)
                .map(|tree| tree.value().query_count(start, end))
                .unwrap_or(0);
            format!("{}\t{}\t{}\t{}\n", chr, start, end, count)
        })
        .collect::<Vec<String>>();

    it.iter().for_each(|x| {
        result.write_all(x.as_bytes()).unwrap();
    });
}

fn build_tree(file: &PathBuf) -> ChrIntervalTree {
    // chr1    181400  181560  .       0       .       0.537938        -1      -1      75

    let file = std::fs::File::open(file).unwrap();
    let content = std::io::BufReader::new(file);

    let nodes: DashMap<String, Vec<Node>> = DashMap::new();

    content.lines().par_bridge().for_each(|x| {
        let line = x.unwrap();
        let mut content = line.split('\t');
        let chr = content.next().unwrap().to_string();
        let start = content.next().unwrap().parse::<i32>().unwrap();
        let end = content.next().unwrap().parse::<i32>().unwrap();
        // nodes.insert(chr, Node::new(start, end, chr));
        nodes
            .entry(chr.to_owned())
            .or_insert_with(Vec::new)
            .push(Node::new(start, end, chr));
    });

    use std::mem;

    DashMap::from_par_iter(nodes.par_iter_mut().map(|mut item| {
        // (item.key(), IntervalTree::new(item.value_mut());
        let chr = item.key().clone();
        let mut node = Vec::new();
        mem::swap(&mut node, item.value_mut());

        (chr, IntervalTree::new(node))
    }))
}

fn main() {
    let args = Args::parse();
    let trees = build_tree(&args.atac);
    query_loop(&args.loops, &trees);
    query_boundary(&args.boundary, &trees);
}
