# 2023_DGP_486

## Assignment 1

- [x] Process GSE120024 (human expression data, 7 cases vs. 8 controls) – DESeq2
- [x] Display a PCA plot using all summarized genes
- [x] Display a heatmap of the samples using the top differential genes (eg top 100)
- [x] Evaluate whether there are any enriched pathways/Gene Ontology biological processes using NIH/DAVID.

## Assignment 2

### Part 1:

- [ ] Use one paragraph to describe the biological system in your project and explain the goal of identifying epigenomic differences.
- [ ] Process the ATAC-seq data (fastQ files) and report mapping statistics.
- [ ] Identify tissue-specific or condition-specific ATAC-Seq peaks (submit two bed files, one for each condition).
- [ ] Draw heatmap for the tissue-specific ATAC-Seq peaks.
- [ ] Perform motif search in tissue/condition-specific open chromatin regions and report the top five motifs.
- [ ] Use one paragraph to explain whether the TFs identified in the tissue-specific regions are relevant to the tissue/condition.
- [ ] You just need to pick one TF from each condition and the claims needs to be support by current literature.

Note:

The assignment is due 5 pm on Monday 02/27. NO LATE HOMEWORK WILL BE ACCEPTED.
The second part of the assignment will be posted on 02/20 after we cover the single cell genomics.
Create a folder (Project 2) in your folder on e31900 allocation and store all your results in this folder. We will examine the script and all the intermediate steps.

### Part 2 scRNA-Seq data analysis.

We have mapped the scRNA-seq in PBMC and the output is in /projects/e31900/resource/fastq/scRNA-seq-2/PBMC_output/outs.

Your assignment is to visualize the cloupe file and annotate each cluster.

- [ ] Describe how you annotate the cluster and the literature that support the annotation.
- [ ] Describe why scRNA is important for the paper you chose.
- [ ] Have the authors used the technique to discover something that won’t be possible with bulk assay?
- [ ] What are the validation experiments in the paper to support the finding by scRNA-Seq?

3D genome organization. In folder: /projects/e31900/resource/hi-c, we have the following files:

- TAD boundary file (HAP-1.boundary.bed)
- Chromatin loop file (HAP-1.loops.bedpe)
- ATAC-Seq peak file: (HAP-1-ATAC-seq.peak.bed)

Please compute how many ATAC-Seq peaks are located in a loop anchor and how many of the ATAC-Seq peaks are located in a TAD boundary.
Describe how you compute the overlaps step by step.

Note: each row in HAP-1.loops.bedpe is a loop that containing two anchors.
The first three columns are anchor one and the next three columns are anchor two.

## Final Exam

### Part 1: Through analyzing publicly available dataset, such as The Cancer Genome Atlas, GTEX or ENCODE, to acquire preliminary results and design a project.

Your previously submitted proposals, e.g., the one used in your qualifying exam, are NOT allowed. Submit a two-page brief report with specific aims and summary of preliminary analysis and send it to wei.zhang1@northwestern and yue@northwestern.edu by 5 pm 03/12 (Sunday). Late submission will not be accepted.

Some preliminary analysis result include:

Genes and/or cis-regulatory elements associated with a particular cancer type;
Pathways dysregulated in a particular cancer type relative to other cancer types.
Differential genes/epigenome/3D genome organization associated with treatment response in a particular cancer.

Evaluation will be based on:

1. the significance and novelty of biological/clinical question described in the specific aim page;
2. whether the bioinformatic and experimental design is proper and justified; 3) depth of analysis in the preliminary results.
   We would encourage multi-omics approaches.

### Part 2 - Oral presentation of the results on 03/08 (a ppt talk, ~10 min/student, 8 min presentation + 2 min Q&A );

Evaluation will be based on the clarity of the presentation.
