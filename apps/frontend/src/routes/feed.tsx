import { useEffect, useState } from "react"
import type { ExternalCardData } from "../models"
import ExternalCard from "../components/external_card"
import getDiscoverFeedBatch from "../api/feed/discover"

function Feed() {
	const [cards, setCards] = useState<ExternalCardData[]>([])

	useEffect(() => {
		const getCards = async () => {
			const cards = await getDiscoverFeedBatch()
			setCards(cards)
		}

		getCards()
	}, [])

	return (
		<div>
			{cards.map((c) => (
				<ExternalCard data={c} />
			))}
		</div>
	)
}

export default Feed
