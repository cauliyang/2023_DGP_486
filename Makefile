.PHONY: help
help: ## This help message
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"


assign1: ## Run assignment 1
	./src/assignment1/clean.py data/assignment1/refGene_counts.txt  data/assignment1/GSE120024_series_matrix.txt                                  
	/home/ylk4626/miniconda3/envs/dgp/bin/Rscript ./src/assignment1/deseq.r
	cp ./data/assignment1/*png ./report/assignment1/figures/
 





