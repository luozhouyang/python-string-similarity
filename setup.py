import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="strsimpy",
    version="0.3.0",
    description="A library implementing different string similarity and distance measures",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/luozhouyang/python-string-similarity",
    author="ZhouYang Luo",
    author_email="zhouyang.luo@gmail.com",
    packages=setuptools.find_packages(),
    include_package_data=True,
    install_requires=[],
    license="MIT License",
    classifiers=(
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Python :: 3.13",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    )
)
