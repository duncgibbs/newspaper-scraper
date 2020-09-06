const OliveApi = require("../apis/olive-software");

const downloadRandomPapers = async (newspaper, years, papersPerYear = 1) => {
    const olive = new OliveApi(newspaper);

    for await (const year of years) {
        for (let i = 0; i < papersPerYear; i++) {
            const monthNum = Math.ceil(Math.random() * 12);
            const dayNum = Math.ceil(Math.random() * 31);
            const day = `0${dayNum}`.slice(-2);
            const month = `0${monthNum}`.slice(-2);

            const path = `./random_newspapers/${year}/${month}/DOK_${year}_${month}_${day}.pdf`;
            const pdfName = path.split('/').pop();
        
            olive.getPdfIdFromDate(year, month, day).then(pdfId => {
                olive.downloadPdf(pdfId, path).then(response => {
                    response.on('finish', () => {
                        console.log(`Downloaded ${path}`);
                    });
                }).catch(() => {
                    console.log(`Error downloading PDF: ${pdfName} (${pdfId})`);
                });
            }).catch(() => {
                console.log(`Error getting the pdf id for ${[year, month, day].join('/')}`);
                downloadRandomPapers(newspaper, [year], papersPerYear);
            });
        }
    }
}

module.exports = downloadRandomPapers;