const { createWriteStream } = require('fs');

// https://digital.olivesoftware.com/olive/odn/oklahoman/get/prxml.ashx?kind=doc&href=dok/1901/10/01
const axios = require('axios').default;
const fs = require('fs').promises;

module.exports = class OliveApi {
    constructor (newspaper) {
        this.oliveUrl = `https://digital.olivesoftware.com/olive/apa/${newspaper}`;
    }
    
    getPdfIdFromDate(year, month, day) {
        const getIdPath = `/get/prxml.ashx?kind=doc&href=dok/${year}/${month}/${day}`;

        return axios.get(`${this.oliveUrl}${getIdPath}`).then(response => {
            const pdfId = response.data.pdf;
            return pdfId;
        });
    }

    downloadPdf(pdfId, path) {
        const downloadPath = `/get/pdf.ashx?pdf=${encodeURIComponent(pdfId)}`;
        
        return axios.get(
            `${this.oliveUrl}${downloadPath}`,
            {
                responseType: 'stream'
            }
        ).then(response => {
            const directories = path.split('/').slice(0, -1).join('/');
            return fs.mkdir(directories, {recursive: true}).then(() => {
                return response.data.pipe(createWriteStream(path));
            });
        });
    }
};
