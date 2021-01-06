test: data
	pytest -v

data/reference_image.png:
	python generate_reference_image.py

.PHONY: n5java
n5java: data/reference_image.png
	bash generate_data/n5-java/generate_data.sh

.PHONY: pyn5
pyn5: data/reference_image.png
	python generate_data/generate_pyn5.py

.PHONY: z5py
z5py: data/reference_image.png
	python generate_data/generate_z5py.py

.PHONY: zarr
zarr: data/reference_image.png
	python generate_data/generate_zarr.py

.PHONY: data
data: n5java pyn5 z5py zarr

.PHONY: test

.PHONY: download
download: data/reference_image.png
	    bash read_from_s3/download_froms3.sh
