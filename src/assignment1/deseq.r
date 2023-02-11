#!/usr/bin/env Rscript
# @author: YangyangLi
# @contact: yangyang.li@northwestern.edu

library("DESeq2")
library("pheatmap")
library("ggplot2")


read_count_matrix_and_sample_info <- function(count_file, sample_file) {
  cts <- read.table(count_file, header = T, row.names = 1)
  sampleInfo <- read.table(sample_file, header = T, row.names = 1)
  sampleInfo$group <- factor(sampleInfo$group, levels = c("control", "T2D"))
  sampleInfo$gender <- factor(sampleInfo$gender, levels = c("F", "M"))
  sampleInfo$batch <- factor(sampleInfo$batch, levels = c("A", "B", "C"))
  list(cts = cts, sampleInfo = sampleInfo)
}

deseq2_analysis <- function(cts, sampleInfo, design, pvalue_cutoff) {
  # create DESeqDataSet
  stopifnot(identical(row.names(sampleInfo), colnames(cts)))
  dds <- DESeqDataSetFromMatrix(countData = cts, colData = sampleInfo, design = design)
  
  # filter lowly expressed genes
  keep <- rowSums(counts(dds)) >= 10
  dds <- dds[keep,]
  
  # differential expression analysis
  dds <- DESeq(dds)
  res <- results(dds)
  res <- as.data.frame(res)
  
  # count the number of genes with adjusted p-value less than pvalue_cutoff
  num_genes_less_than = sum(res$padj < pvalue_cutoff, na.rm = TRUE)
  # order by raw p-value
  resOrdered <- res[order(res$pvalue, decreasing = FALSE), ]
  
  # data transformation and visualization
  vsd <- vst(dds, blind = FALSE)
  rld <- rlog(dds, blind = FALSE)
  ntd <- normTransform(dds)
  normCount <- counts(dds, normalized = TRUE)
  
  list(dds = dds, resOrdered = resOrdered, vsd = vsd, rld = rld, ntd = ntd, normCount = normCount, num_genes_less_than = num_genes_less_than)
}

create_plots_and_save <- function(vsd, rld, dds, resOrdered) {
  # PCA plot by group
  plotPCA(vsd, intgroup="group")
  ggsave("pca_by_group.png", plot = last_plot(), width = 7, height = 7, dpi = 300)
  
  # PCA plot by batch
  pca2 <- plotPCA(rld, intgroup="batch")
  ggsave("pca_by_batch.png", plot = last_plot(), width = 7, height = 7, dpi = 300)
  
  # Heatmap of all genes
  df <- as.data.frame(colData(dds))[, c("group", "batch")]
  all_genes_plot <- pheatmap(assay(vsd), cluster_names=TRUE, show_rownames=FALSE, cluster_cols = TRUE, annotation_col=df)
  ggsave("all_genes_plot.png", plot = all_genes_plot, width = 7, height = 7, dpi = 300)
  
  # Heatmap of selected genes
  select <- rownames(resOrdered)[1:10]
  df_select <- as.data.frame(colData(dds))[, c("group", "batch")]
  selected_gene_plot <- pheatmap(assay(vsd)[select,], cluster_rows=TRUE, show_rownames=FALSE, cluster_cols=TRUE, annotation_col=df_select)
  ggsave("selected_gene_plot.png", plot = selected_gene_plot, width = 7, height = 7, dpi = 300)
}

main <- function() {

  setwd("/projects/e31900/yangyangli/data/assignment1/")
  data <- read_count_matrix_and_sample_info("cleaned_count.txt", "cleaned_sample.txt")
  
  cts <- data$cts
  sampleInfo <- data$sampleInfo

  # group batch gender
  design <- ~ group

  results <- deseq2_analysis(cts, sampleInfo, design, 0.05)
  
  resOrdered <- results$resOrdered
  vsd <- results$vsd
  rld <- results$rld
  ntd <- results$ntd
  normCount <- results$normCount
  num_genes_less_than <- results$num_genes_less_than
  create_plots_and_save(vsd, rld, results$dds, resOrdered)
}


main()
