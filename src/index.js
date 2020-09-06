const range = (min, max) => {
    let results = [];
    for (let i = min; i < max; i++) {
        results.push(i);
    }
    return results;
};

const years = range(1901, 1951);

// const downloadYears = require('./scrapers/download-pdfs');
// downloadYears('oklahoman', years);


const randomPapers = require('./scrapers/random-downloader');
randomPapers('oklahoman', years);
