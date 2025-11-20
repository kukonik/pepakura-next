import React, { useRef, useState } from 'react';

const API_URL = 'http://localhost:8080/unfold'; // unfolding-core Rust backend

export default function DragDropUpload() {
    const fileInputRef = useRef<HTMLInputElement>(null);
    const [drag, setDrag] = useState(false);
    const [resultUrl, setResultUrl] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);

    async function handleFiles(files: FileList) {
        if (!files.length) return;
        const file = files[0];
        const form = new FormData();
        form.append('file', file);
        try {
            const res = await fetch(API_URL, {
                method: 'POST',
                body: form
            });
            if (!res.ok) throw new Error(`API error: ${res.status}`);
            // допустим сервер отдаёт pdf/svg — временно заботимся о бинарном ответе
            const blob = await res.blob();
            const url = URL.createObjectURL(blob);
            setResultUrl(url);
            setError(null);
        } catch (e: any) {
            setError(e.message);
            setResultUrl(null);
        }
    }

    function handleDrop(e: React.DragEvent) {
        e.preventDefault();
        setDrag(false);
        handleFiles(e.dataTransfer.files);
    }

    function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
        if (e.target.files) {
            handleFiles(e.target.files);
        }
    }

    return (
        <div
            onDragOver={e => {e.preventDefault();setDrag(true);}}
            onDragLeave={() => setDrag(false)}
            onDrop={handleDrop}
            style={{
                border: '2px dashed #888',
                padding: 32,
                textAlign: 'center',
                background: drag ? '#e6f7ff' : 'white',
                borderRadius: 8
            }}
        >
            <h2>Загрузка 3D модели (.obj/.stl)</h2>
            <input
                type="file"
                accept=".obj,.stl"
                ref={fileInputRef}
                style={{display:'none'}}
                onChange={handleChange}
            />
            <button onClick={() => fileInputRef.current?.click()}>Выбрать файл</button>
            <div style={{marginTop:16, color:'#a00'}}>{error}</div>
            {resultUrl && (
                <div style={{marginTop:16}}>
                    <a href={resultUrl} download="unfolded.pdf">Скачать раскладку</a>
                    <iframe src={resultUrl} title="Preview" width={400} height={300}/>
                </div>
            )}
        </div>
    );
}
