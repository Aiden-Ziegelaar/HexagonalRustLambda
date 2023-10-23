import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Delete Product', function () {
    it('should delete a created product', async function () {
        //arrange
        let product = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        //act
        let res_post = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)

        let res_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/product/${res_post.data.id}`)

        //assert
        assert.equal(res_delete.status, 200)
        expect(res_delete.data).to.include(product)
    })

    it('should fail to delete a nonexistant product', async function () {
        //arrange
        let id = faker.string.uuid()

        //act
        let res_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/product/${id}`,
            {
                validateStatus: () => true
            }
        )

        //assert
        assert.equal(res_delete.status, 404)
    })
});