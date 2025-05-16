

function setupFileLoad(){
    document.querySelector('#image-upload').addEventListener('change', function() {
        if (this.files && this.files[0]) {
            let img = document.querySelector('#image-display');
            img.onload = () => {
                URL.revokeObjectURL(img.src);  // no longer needed, free memory
            }

            img.src = URL.createObjectURL(this.files[0]); // set src to blob url
        }
    });
}

window.onload = ()=>{
    setupFileLoad()
}