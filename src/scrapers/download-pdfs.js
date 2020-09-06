const OliveApi = require("../apis/olive-software");

module.exports = async (newspaper, years) => {
    const olive = new OliveApi(newspaper);

    for await (const year of years) {
        for await (const monthNum of [...Array(13).keys()].slice(1)) {
            for (let dayNum = 1; dayNum < 32; dayNum++) {
                const day = `0${dayNum}`.slice(-2);
                const month = `0${monthNum}`.slice(-2);
    
                const path = `./newspapers/${year}/${month}/DOK_${year}_${month}_${day}.pdf`;
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
                });
            }
        }
    }
}
