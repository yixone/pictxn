import type { ExternalCardData } from "../models"

export default function ExternalCard({ data }: { data: ExternalCardData }) {
	return (
		<div>
			<img className="w-24" src={data.preview_url ?? data.file_url} />
		</div>
	)
}
