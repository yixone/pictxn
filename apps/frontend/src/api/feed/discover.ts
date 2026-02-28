import type ExternalCardData from "../../models/external_card"
import { API_URL } from "../api"

async function getDiscoverFeedBatch(): Promise<ExternalCardData[]> {
	const res = await fetch(`${API_URL}/feed/discover`)

	if (!res.ok) {
		throw new Error(await res.text())
	}

	return await res.json()
}

export default getDiscoverFeedBatch
